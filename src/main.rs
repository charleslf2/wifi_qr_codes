use eframe::egui;
use wifi_qr_code::{AuthenticationType, Visibility, WifiCredentials};
use wifi_qr_code::QrCodeEcc;
use std::fs::File;


fn main() {
    let native_options=eframe::NativeOptions{
        initial_window_size:Some(egui::vec2(300.0, 400.0)),
        resizable:false,
        maximized:false,
        ..Default::default()
    };

    eframe::run_native(
        "Wifi qr codes",
        native_options, 
        Box::new(|_cc|Box::<Wifi>::default()),
    );

}


struct Wifi{
    name:String,
    auth_type:String,
    password:String,  
}

impl Wifi{
    fn  generateqrcode(&mut self){
        let wifi_cred=WifiCredentials{
            ssid:String::from(self.name.to_owned()),
            //if self.auth_type =="WPA"{
            authentication_type:AuthenticationType::WPA(String::from(self.password.to_owned())),
           // }
            visibility:Visibility::Visible,
        };
        let png_file=File::create("wifi_qr.png").expect("failed");
        let image= wifi_qr_code::encode_as_png(&wifi_cred, QrCodeEcc::Medium, 200, png_file);

        println!("hello world");
    }
}

impl Default for Wifi{
    fn default()->Self{
        Self{
            name:"CANALBOX-B9F7-2G".to_owned(),
            auth_type:"WPA".to_owned(),
            password:"1234".to_owned(),

        }
    }
}

impl eframe::App for Wifi{
    fn update(&mut self, ctx:&egui::Context, _frame:&mut eframe::Frame){
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.label("Wifi Name");
            ui.text_edit_singleline(&mut self.name);
            ui.label("Authentication type");
            ui.text_edit_singleline(&mut self.auth_type);
            ui.label("The Password");
            ui.text_edit_singleline(&mut self.password);

            if ui.button("generate").clicked(){
                Wifi::generateqrcode(self);
            }
        });
    }
}









