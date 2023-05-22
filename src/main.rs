use eframe::egui;
use wifi_qr_code::{AuthenticationType, Visibility, WifiCredentials};
use wifi_qr_code::QrCodeEcc;
use std::fs::File;
use std::path::PathBuf;


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
    password:String,  
    image_name:String,
    dir_path:String,
}

impl Wifi{
    fn  generateqrcode(&mut self){
        let wifi_cred=WifiCredentials{
            ssid:String::from(self.name.to_owned()),
            authentication_type:AuthenticationType::WPA(String::from(self.password.to_owned())),
            visibility:Visibility::Visible,
        };
        let png_file=File::create(self.dir_path.to_owned() + &self.image_name.to_owned()+ ".png").expect("failed");
        let image= wifi_qr_code::encode_as_png(&wifi_cred, QrCodeEcc::Medium, 200, png_file);
        //let dir_file=PathBuf::new().push(self.dir_path +png_file);

        println!("hello world");
    }
}

impl Default for Wifi{
    fn default()->Self{
        Self{
            name:"CANALBOX-B9F7-2G".to_owned(),
            password:"1234".to_owned(),
            image_name:"new_qr_code".to_owned(),
            dir_path:"C:/Users/Charles_lf/Desktop".to_owned(),

        }
    }
}

impl eframe::App for Wifi{
    fn update(&mut self, ctx:&egui::Context, _frame:&mut eframe::Frame){
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.label("Wifi Name");
            ui.text_edit_singleline(&mut self.name);
            ui.label("The Password");
            ui.text_edit_singleline(&mut self.password);
            ui.label("saved path");
            ui.text_edit_singleline(&mut self.dir_path);
            ui.label("qr code name");
            ui.text_edit_singleline(&mut self.image_name);

            if ui.button("generate").clicked(){
                Wifi::generateqrcode(self);
            }
        });
    }
}









