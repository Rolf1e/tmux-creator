use crate::event::EventResponse;
use crate::event_handler::EventHandler;
use crate::exception::NeovimException;
use crate::window_builder::Type;
use crate::window_builder::Window;
use async_trait::async_trait;
use nvim_rs;
use nvim_rs::compat::tokio::Compat;
use nvim_rs::create::tokio as create;
use nvim_rs::error;
use nvim_rs::Value;
use rmpv;
use std::str::FromStr;
use tmux_lib::logger;
use tokio::io;
use tokio::task;

type Writer = Compat<io::Stdout>;

pub async fn build_neovim() -> (
    nvim_rs::Neovim<Compat<io::Stdout>>,
    task::JoinHandle<Result<(), Box<error::LoopError>>>,
) {
    create::new_parent(NeovimHandler::new()).await
}

#[derive(Clone)]
pub struct NeovimHandler {
    event_handler: EventHandler,
    neovim_window_handler: NeovimWindowHandler,
}

#[derive(Clone)]
pub struct NeovimWindowHandler;

#[async_trait]
impl nvim_rs::Handler for NeovimHandler {
    type Writer = Writer;

    async fn handle_request(
        &self,
        name: String,
        args: Vec<nvim_rs::Value>,
        neovim: nvim_rs::Neovim<Self::Writer>,
    ) -> Result<nvim_rs::Value, nvim_rs::Value> {
        let (ui_width, ui_height) = self.get_ui_size(&neovim).await?;
        let event_response = self.event_handler.handle_event(name, args, (ui_width, ui_height));
        if let EventResponse::Window(ref window) = event_response {
            if let Err(e) = self
                .neovim_window_handler
                .write_to_popup_window(neovim, window)
                .await
            {
                logger::error(&e.message());
            }
        }
        event_response.to_neovim()
    }
}

impl NeovimWindowHandler {

    pub async fn write_to_popup_window(
        &self,
        neovim: nvim_rs::Neovim<Writer>,
        window: &Window,
    ) -> Result<(), NeovimException> {
        let buffer = self.create_buffer(&neovim).await?;
        self.add_lines_to_buffer(&buffer, window.get_text().clone()).await?;

        for key_map in window.get_key_maps() {
            self.add_key_map(
                &buffer,
                key_map.get_mode(), 
                key_map.get_mapping(),
                key_map.get_command(),
                key_map.get_opts().clone(),
            )
                .await?;
        }

        let opts: Vec<(nvim_rs::Value, nvim_rs::Value)> = window.get_ui_settings()
            .into_iter()
            .map(|(key, value)| (nvim_rs::Value::from(key.clone()), Self::parse_type_to_value(value)))
            .collect();

        if let Err(e) = neovim.open_win(&buffer, true, opts).await {
            Err(NeovimException::WindowCreation(e))
        } else {
            Ok(())
        }

    }

    fn parse_type_to_value(t: &Type) ->  nvim_rs::Value {
        match t {
            Type::Integer(v) => nvim_rs::Value::Integer(rmpv::Integer::from(v.clone())),
            Type::String(v) => nvim_rs::Value::from(v.clone()),
        }
    }

    async fn create_buffer(
        &self,
        neovim: &nvim_rs::Neovim<Writer>,
    ) -> Result<nvim_rs::Buffer<Writer>, NeovimException> {
        match neovim.create_buf(false, true).await {
            Ok(buffer) => Ok(buffer),
            Err(e) => Err(NeovimException::WindowCreation(e)),
        }
    }

    async fn add_lines_to_buffer(
        &self,
        buffer: &nvim_rs::Buffer<Writer>,
        text: Vec<String>,
    ) -> Result<(), NeovimException> {
        if let Err(e) = buffer.set_lines(0, -1, true, text).await {
            Err(NeovimException::WindowCreation(e))
        } else {
            Ok(())
        }
    }

    async fn add_key_map(
        &self,
        buffer: &nvim_rs::Buffer<Writer>,
        mode: &str,
        closing_keys: &str,
        rhs: &str,
        opts: Vec<(String, bool)>,
    ) -> Result<(), NeovimException> {
        let opts = opts
            .into_iter()
            .map(|(key, value)| (nvim_rs::Value::from(key), nvim_rs::Value::Boolean(value)))
            .collect();
        if let Err(e) = buffer.set_keymap(mode, closing_keys, rhs, opts).await {
            Err(NeovimException::WindowCreation(e))
        } else {
            Ok(())
        }
    }

}

impl NeovimHandler {
    fn new() -> Self {
        NeovimHandler {
            event_handler: EventHandler::default(),
            neovim_window_handler: NeovimWindowHandler,
        }
    }

    async fn get_ui_size(&self, neovim: &nvim_rs::Neovim<Writer>) -> Result<(i64, i64), nvim_rs::Value> {
        let data = match self.get_first_uis(&neovim).await {
            Ok(data) => data,
            Err(e) => return Err(nvim_rs::Value::from(e.message())),
        };

        let ui_width: i64 = match Self::extract_data_from_value_map(&data, "width") {
            Ok(ui_width)=> ui_width,
            Err(e) => return Err(nvim_rs::Value::from(e.message())),
        };
        let ui_height: i64 = match Self::extract_data_from_value_map(&data, "height") {
            Ok(ui_height)=> ui_height,
            Err(e) => return Err(nvim_rs::Value::from(e.message())),
        };

        Ok((ui_width, ui_height))
    }

    async fn list_uis(
        &self,
        neovim: &nvim_rs::Neovim<Writer>,
    ) -> Result<Vec<Value>, NeovimException> {
        match neovim.list_uis().await {
            Ok(values) => Ok(values),
            Err(e) => Err(NeovimException::WindowCreation(e)),
        }
    }

    async fn get_first_uis(
        &self,
        neovim: &nvim_rs::Neovim<Writer>,
    ) -> Result<Vec<(String, String)>, NeovimException> {
        let ui = &self.list_uis(neovim).await?[0];
        if let Some(data) = ui.as_map() {
            let mut final_data = Vec::new();
            for (key, value) in data {
                final_data.push((Self::parse_key(key)?, value.to_string()));
            }
            Ok(final_data)
        } else {
            logger::error(&format!(
                "Failed to read data from ui. \n {}",
                ui.to_string()
            ));
            let e = error::CallError::WrongValueType(ui.clone());
            Err(NeovimException::WindowCreation(Box::new(e)))
        }
    }

    // I'm doing this because to_string add '\', and that's anoying
    fn parse_key(value: &Value) -> Result<String, NeovimException> {
        if let Some(value) = value.as_str() {
            Ok(String::from(value))
        } else {
            Err(NeovimException::Convertion(value.to_string()))
        }
    }

    fn extract_data_from_value_map<T: FromStr>(
        data: &Vec<(String, String)>,
        key: &str,
    ) -> Result<T, NeovimException> {
        if let Some((_, value)) = data.into_iter().find(|(i_key, _)| i_key == key) {
            Self::parse_neovim_value(&value)
        } else {
            Err(NeovimException::Extract(String::from(key)))
        }
    }

    fn parse_neovim_value<T: FromStr>(value: &String) -> Result<T, NeovimException> {
        if let Ok(value) = value.parse() {
            Ok(value)
        } else {
            Err(NeovimException::Convertion(String::from(value)))
        }
    }
}

impl Default for NeovimHandler {
    fn default() -> Self {
        Self::new()
    }
}
