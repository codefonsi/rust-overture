use rust_overture::{concat::*, concat_fn, concat_mut_macro, concat_tryfn, concat_trymut};

#[derive(Debug)]
enum Error {
    some_error,
}
fn main() {
    // --- concat_fn! ---
    let f = concat_fn!(|x: i32| x + 1, |x| x * 2, |x| x - 3);
    println!("concat_fn!(5) => {}", f(5)); // ((5 + 1) * 2) - 3 = 9

    let f_single = concat_fn!(|x: i32| x * 10);
    println!("concat_fn!(7) => {}", f_single(7)); // 70

    // --- concat_tryfn! ---
    let f_try = concat_tryfn!(|x: i32| Ok(x + 1), |x| Ok(x * 2), |x| if x > 10 {
        Err("too big")
    } else {
        Ok(x - 3)
    });
    println!("concat_tryfn!(5) => {:?}", f_try(5)); // Ok(((5+1)*2)-3) = Ok(9)
    println!("concat_tryfn!(20) => {:?}", f_try(20)); // Err("too big")

    // let f_try_single = concat_tryfn!(|x: i32| Ok(x * 100));
    let f_try_single = concat_tryfn!(|x: i32| Ok::<_, Error>(x * 100));
    println!("concat_tryfn!(2) => {:?}", f_try_single(2)); // Ok(200)

    // --- concat_mut! ---
    let mut f_mut = concat_mut_macro!(
        |x: &mut i32| *x += 2,
        |x: &mut i32| *x *= 3,
        |x: &mut i32| *x -= 5
    );
    let mut val = 4;
    f_mut(&mut val); // ((4+2)*3)-5 = 13
    println!("concat_mut!(4) => {}", val);

    let mut f_mut_single = concat_mut_macro!(|x: &mut i32| *x *= 10);
    let mut val2 = 6;
    f_mut_single(&mut val2);
    println!("concat_mut!(6) => {}", val2); // 60

    // --- concat_trymut! ---
    let mut f_trymut = concat_trymut!(
        |x: &mut i32| {
            *x += 2;
            Ok(())
        },
        |x: &mut i32| {
            *x *= 3;
            Ok(())
        },
        |x: &mut i32| { if *x > 50 { Err("too big") } else { Ok(()) } }
    );
    let mut val3 = 5;
    println!("concat_trymut!(5) => {:?}", f_trymut(&mut val3)); // Ok, val3 = ((5+2)*3) = 21
    println!("val3 after => {}", val3);

    let mut val4 = 20;
    println!("concat_trymut!(20) => {:?}", f_trymut(&mut val4)); // Err("too big"), stops early
    println!("val4 after => {}", val4); // still modified up to failure point

    let mut f_trymut_single = concat_trymut!(|x: &mut i32| {
        *x *= 2;
        Ok::<(), Error>(())
    });
    let mut val5 = 9;
    println!("concat_trymut!(9) => {:?}", f_trymut_single(&mut val5)); // Ok, val5 = 18
    println!("val5 after => {}", val5);
}
