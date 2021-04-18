use crate::event::EventResponse;
use crate::event_handler::EventHandler;
use crate::exception::NeovimException;
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
struct NeovimHandler {
    event_handler: EventHandler,
}

#[derive(Clone)]
struct WindowMaker {
    neovim: nvim_rs::Neovim<Writer>,
}

impl NeovimHandler {
    fn new() -> Self {
        NeovimHandler {
            event_handler: EventHandler::default(),
        }
    }
}

#[async_trait]
impl nvim_rs::Handler for NeovimHandler {
    type Writer = Writer;

    async fn handle_request(
        &self,
        name: String,
        args: Vec<nvim_rs::Value>,
        neovim: nvim_rs::Neovim<Self::Writer>,
    ) -> Result<nvim_rs::Value, nvim_rs::Value> {
        let event_response = self.event_handler.handle_event(name, args);
        if let EventResponse::Window(ref text) = event_response {
            let window_maker = WindowMaker { neovim };
            if let Err(e) = window_maker.write_to_popup_window(text.clone()).await {
                logger::error(&e.message());
            }
        }
        event_response.to_neovim()
    }
}

impl WindowMaker {
    pub async fn write_to_popup_window(&self, text: Vec<String>) -> Result<(), NeovimException> {
        let buffer = self.create_buffer().await?;
        self.add_lines_to_buffer(&buffer, text).await?;
        let opts = vec![
            (String::from("silent"), true),
            (String::from("nowait"), true),
            (String::from("noremap"), true),
        ];
        self.add_key_map(&buffer, "n", "q", ":close<CR>", opts.clone())
            .await?;

        self.add_key_map(&buffer, "n", "<CR>", ":LaunchSession <C-R><C-W> <CR>", opts)
            .await?;
        let width: i64 = 50;
        let height: i64 = 50;

        let data = self.get_first_uis().await?;

        let ui_width: i64 = Self::extract_data_from_value_map(&data, "width")?;
        let ui_height: i64 = Self::extract_data_from_value_map(&data, "height")?;

        let opts = vec![
            (
                nvim_rs::Value::from("relative"),
                nvim_rs::Value::from("editor"),
            ),
            (
                nvim_rs::Value::from("width"),
                nvim_rs::Value::Integer(rmpv::Integer::from(width)),
            ),
            (
                nvim_rs::Value::from("height"),
                nvim_rs::Value::Integer(rmpv::Integer::from(height)),
            ),
            (
                nvim_rs::Value::from("col"),
                nvim_rs::Value::Integer(rmpv::Integer::from((ui_width / 2) - (width / 2))),
            ),
            (
                nvim_rs::Value::from("row"),
                nvim_rs::Value::Integer(rmpv::Integer::from((ui_height / 2) - (height / 2))),
            ),
            (nvim_rs::Value::from("anchor"), nvim_rs::Value::from("NW")),
            (
                nvim_rs::Value::from("style"),
                nvim_rs::Value::from("minimal"),
            ),
            (
                nvim_rs::Value::from("border"),
                nvim_rs::Value::from("single"),
            ),
        ];

        if let Err(e) = self.neovim.open_win(&buffer, true, opts).await {
            Err(NeovimException::WindowCreation(e))
        } else {
            Ok(())
        }
    }

    async fn get_current_line(&self) -> Result<String, NeovimException> {
        match self.neovim.get_current_line().await {
            Ok(line) => Ok(line),
            Err(e) => Err(NeovimException::WindowCreation(e)),
        }

    }


    async fn create_buffer(&self) -> Result<nvim_rs::Buffer<Writer>, NeovimException> {
        match self.neovim.create_buf(false, true).await {
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

    async fn list_uis(&self) -> Result<Vec<Value>, NeovimException> {
        match self.neovim.list_uis().await {
            Ok(values) => Ok(values),
            Err(e) => Err(NeovimException::WindowCreation(e)),
        }
    }

    async fn get_first_uis(&self) -> Result<Vec<(String, String)>, NeovimException> {
        let ui = &self.list_uis().await?[0];
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
