// 填空
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break 20;
        }
    };

    assert_eq!(result, 20);
}
