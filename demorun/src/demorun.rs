use demos::exam11::{world, Hello, Summary};
use demos::exam2::{self, ShirtColor};
use demos::exam5::CustomSmartPointer;
use demos::exam5::MyBox;
use demos::exam7;
use demos::exam8;
use demos::exam9;
use demos::{exam10, exam6};
use demos::{exam3, exam5};

struct Closure<F> {
    data: (u8, u16),
    func: F,
}

impl<F> Closure<F>
where
    F: for<'a> Fn(&'a (u8, u16)) -> &'a u8,
{
    fn call(&self) -> &u8 {
        (self.func)(&self.data)
    }
}

fn do_it(data: &(u8, u16)) -> &u8 {
    &data.0
}

fn main() {
    // let x = exam2::Inventory {
    //     shirts: vec![
    //         ShirtColor::Red,
    //         ShirtColor::Blue,
    //         ShirtColor::Red,
    //         ShirtColor::Blue,
    //         ShirtColor::Blue,
    //     ],
    // };

    // let user_pref1 = Some(ShirtColor::Red);
    // let user1 = x.giveaway(user_pref1);
    // println!("{user1:?}");
    // let user_pref2 = None;
    // let user2 = x.giveaway(user_pref2);
    // println!("{user2:?}");
    // exam3::exam3();
    // exam3::move_borrow();
    // let y = 5;
    // let z = MyBox::new(y);
    // println!("{z:?}");
    // assert_eq!(5, *z);

    // exam5::hello(&MyBox::new(String::from("Yes")));

    // let _m = CustomSmartPointer::new(String::from("Hello World"));
    // exam6::exam6();
    // exam7::exam7();
    // exam7::exam71();
    // exam7::exam72();
    // exam7::exam73();
    // exam7::exam74();
    // exam8::exam();
    // exam9::exam();
    // exam10::exam();

    // let clo = Closure {
    //     data: (255, 1),
    //     func: do_it,
    // };
    // println!("{}", clo.call());
    let hello = Hello::new("Yes".to_string());
    world(&hello);
}
