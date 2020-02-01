
use std::collections::HashSet;
use std::string::String;


#[derive(FromForm, Debug)]
pub struct ClientUrl {
    pub os: String,
    pub service: Option<String>,
    pub ver: String,
    pub df: String,
    pub cver: String,
}


lazy_static! {
    static ref OS_SET: HashSet<&'static str> = {
            let mut set = HashSet::new();
            set.insert("android");
            set.insert("ios");
            set.insert("wp");
            set
    };
    static ref DF_SET: HashSet<&'static str> = {
            let mut set = HashSet::new();
            set.insert("json");
            set.insert("ra");
            set.insert("gz_ra");
            set
    };

}

impl ClientUrl {

    pub fn validate(&self, service_name: &String) -> Option<ClientUrl> {

        let client_url = ClientUrl{
            os: String::from(&self.os),
            service: Some(service_name.clone()),
            ver: String::from(&self.ver),
            df: String::from(&self.df),
            cver: String::from(&self.cver),
        };

        for _os in OS_SET.iter() {
            if _os.eq(&self.os) {
                return Some(client_url);
            }
        }
        for _df in OS_SET.iter() {
            if _df.eq(&self.df) {
                return Some(client_url);
            }
        }

        return None;
    }

    pub fn to_string(&self) -> String{
        let mut s = String::from("[service=");
        s.push_str( self.service.as_ref().unwrap().as_str());
        s.push_str(", ver=");
        s.push_str(self.ver.as_str());
        s.push_str(", os=");
        s.push_str(self.os.as_str());
        s.push_str(", df=");
        s.push_str(self.df.as_str());
        s.push_str(", cver=");
        s.push_str(self.cver.as_str());
        s.push_str("]");

        return s;
    }
}
