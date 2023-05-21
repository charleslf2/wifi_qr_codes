use eframe::egui;
use wifi_qr_code::{AuthenticationType, Visibility, WifiCredentials};


fn main() {
    let native_options=eframe::NativeOptions{
        initial_window_size:Some(egui::vec2(300.0, 200.0)),
        resizable:false,
        fullscreen:false,
        ..Default::default(),
    };

    eframe::run_native(
        "Wifi qr codes",
        native_options, 

    )

}


struct Wifi{
    name:String,
    ahthentification_type:String,
    passeword:String,  
}


impl Default for Wifi{
    fn default()->Self{

    }
}