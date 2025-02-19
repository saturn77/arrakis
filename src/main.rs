pub mod app; 
pub mod components;
pub mod models;
use app::parameters::gui;
use app::app::MyApp;
use app::app::BackroundThread; 

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let background_thread = BackroundThread::new();

    let background_thread_tokio = background_thread.clone();
    let mut local_tic: u32 = 0;

    tokio::task::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            let mut tokio_tic = background_thread_tokio.tic.lock().unwrap();
            *tokio_tic = format!("Tic = {}", local_tic);
            local_tic += 1;
        }
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([gui::VIEWPORT_X, gui::VIEWPORT_Y]),
        ..Default::default()
    };

    eframe::run_native(
        gui::WINDOW_TITLE,
        options,
        Box::new(move |cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MyApp::new(cc, background_thread.clone())))
        }),
    )
}
