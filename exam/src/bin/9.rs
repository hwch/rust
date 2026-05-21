pub trait WeChat {
    fn msg(&self, text: &str) {
        println!("Unkown send message: {}", text);
    }
}

struct WeiXin;
struct QQ;
struct OIQC;

impl WeChat for WeiXin {
    fn msg(&self, text: &str) {
        println!("WeiXin send message: {}", text);
    }
}

impl WeChat for QQ {
    fn msg(&self, text: &str) {
        println!("QQ send message: {}", text);
    }
}

impl WeChat for OIQC {}
fn main() {
    let wx = WeiXin;
    let qq = QQ;
    let oiqc = OIQC;
    wx.msg("Hello");
    qq.msg("World");
    oiqc.msg("Haha");
}
