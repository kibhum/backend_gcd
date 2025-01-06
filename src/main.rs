use actix_web::{web, App, HttpResponse, HttpServer};
use image::{codecs::png::PngEncoder, ExtendedColorType, ImageEncoder, ImageError};
use num::Complex;
use rand::Rng;
use serde::Deserialize;
use std;
use std::fs::File;
use std::io;
use std::thread;
use std::time;
use tokio;

#[derive(Deserialize)]
struct GCDParameters {
    n: u64,
    m: u64,
}

async fn print_odd() {
    let total = 10;
    let mut count = 0;
    let mut odd = 1;
    let three_seconds = time::Duration::from_millis(3000);
    while count < total {
        println!("Odd value: {odd}");
        odd += 2;
        count += 1;
        tokio::time::sleep(three_seconds).await;
    }
}

async fn print_even() {
    let total = 10;
    let mut count = 0;
    let mut even = 0;
    let three_seconds = time::Duration::from_millis(3000);
    while count < total {
        println!("Even value: {even}");
        even += 2;
        count += 1;
        tokio::time::sleep(three_seconds).await;
    }
}

fn func_with_closure<G>(f: G)
where
    G: FnOnce(&str),
{
    f("hello world");
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        // Exchange the values of m, n if m<n
        if m < n {
            let temp = m;
            m = n;
            n = temp;
        }
        m %= n
    }
    n
}

// #[tokio::main]
#[actix_web::main]
async fn main() {
    // let odd = print_odd();
    // let even = print_even();
    // println!("Begin counting...");
    // // This waits until both functions are executed
    // tokio::join!(even, odd);
    // println!("This is the end");
    // let s = "The content of x is: ";
    // let print_x_closure = |x: &str| {
    //     println!("{s} {x}");
    // };
    // func_with_closure(print_x_closure);

    // RUST HTTP SERVER
    // let server = HttpServer::new(|| {
    //     App::new()
    //         .route("/", web::get().to(get_index))
    //         .route("/gcd", web::post().to(post_gcd))
    // });
    // println!("Server is running port 3000");
    // server
    //     .bind("127.0.0.1:3000")
    //     .expect("Error binding to the port")
    //     .run()
    //     .await
    //     .expect("Error running the server");
    // Multi-Threading Exploration
    // run_thread();
    // let res = test_question_mark_operator();
    // match res {
    //     Ok(s) => {
    //         println!("File Opened")
    //     }
    //     Err(e) => eprintln!("{:?}", e),
    // }
    // let png_width = 640;
    // let png_height = 480;
    // let file_name = "gray.png";
    // let mut image_buffer = vec![0; png_width * png_height];
    // for idx in 0..png_width * png_height {
    //     image_buffer[idx] = rand::thread_rng().gen_range(0..=255);
    // }
    // let write_res = write_png(file_name, &image_buffer, (png_width, png_height));
    // match write_res {
    //     Ok(_) => {
    //         println!("Gray Png created!")
    //     }
    //     Err(e) => {
    //         println!("Error creating png: {:?}", e)
    //     }
    // }
    run_in_single_thread();
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
        <title> GCD Calculator</title>
        <form action="/gcd" method="post">
        <input type="text" name="n"/>
        <input type="text" name="m"/>
        <button type="submit">Compute GCD</button>
        </form>
        "#,
    )
}

async fn post_gcd(form: web::Form<GCDParameters>) -> HttpResponse {
    if form.m == 0 || form.n == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with ZERO is boring");
    }

    let response = format!(
        "The greatest common divisor of {} and {} is <b>{}</b>",
        form.m,
        form.n,
        gcd(form.m, form.n)
    );
    HttpResponse::Ok().content_type("text/html").body(response)
}

fn run_thread() {
    let s = "String in the run thread";
    let mut v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        thread::sleep(time::Duration::from_secs(2));
        println!("The Content of s is: {}", s);
        v.push(4);
        println!("The Content of V is: {:?}", v);
    });
    handle.join().unwrap();
}

fn test_question_mark_operator() -> Result<String, std::io::Error> {
    std::fs::File::open("non_existent.txt")?;
    // let res = std::fs::File::open("non_existent.txt");
    // match res {
    //     Ok(file) => {
    //         return Ok("File opened successfully: {}".to_string());
    //     }
    //     Err(e) => Err(e),
    // }
    Ok("File opened successfully: {}".to_string())
}

fn write_png(file_name: &str, pixels: &[u8], dimensions: (usize, usize)) -> Result<(), ImageError> {
    let output = File::create(file_name)?;
    let encoder = PngEncoder::new(output);
    encoder.write_image(
        pixels,
        dimensions.0 as u32,
        dimensions.1 as u32,
        ExtendedColorType::L8,
    )?;
    Ok(())
}

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

#[test]
fn test_points_for_mandelbrot_set() {
    let limit = 255;
    // O is in the set
    assert!(escape_time(Complex { re: 0.0, im: 0.0 }, limit).is_none());
    // -1 is in the set
    assert!(escape_time(Complex { re: -1.0, im: 0.0 }, limit).is_none());
    // i is in the set
    assert!(escape_time(Complex { re: 0.0, im: 1.0 }, limit).is_none());
    // 2i is not in the set
    assert!(escape_time(Complex { re: 0.0, im: 2.0 }, limit).is_some());
    // 3 is not in the set
    assert!(escape_time(Complex { re: 3.0, im: 0.0 }, limit).is_some());
    // 1 is not in the set
    assert!(escape_time(Complex { re: 1.0, im: 0.0 }, limit).is_some());
    // 1+1 is not in the set
    assert!(escape_time(Complex { re: 1.0, im: 1.0 }, limit).is_some());
}

fn pixel_to_complex_number(
    image_dimension: (usize, usize),
    pixel_coordinates: (usize, usize),
    complex_upper_left: Complex<f64>,
    complex_bottom_right: Complex<f64>,
) -> Complex<f64> {
    let complex_plane_width = complex_bottom_right.re - complex_upper_left.re;
    let complex_plane_height = complex_upper_left.im - complex_bottom_right.im;
    Complex {
        re: complex_upper_left.re
            + (pixel_coordinates.0 as f64 / image_dimension.0 as f64) * complex_plane_width,
        im: complex_upper_left.im
            - (pixel_coordinates.1 as f64 / image_dimension.1 as f64) * complex_plane_height,
    }
}

#[test]
fn test_pixel_to_complex_number() {
    assert_eq!(
        pixel_to_complex_number(
            (100, 100),
            (20, 30),
            Complex { re: 10.0, im: 20.0 },
            Complex { re: 20.0, im: 5.0 },
        ),
        Complex { re: 12.0, im: 15.5 }
    )
}

fn render(
    pixels: &mut [u8],
    image_dimension: (usize, usize),
    complex_upper_left: Complex<f64>,
    complex_bottom_right: Complex<f64>,
) {
    assert!(pixels.len() == image_dimension.0 * image_dimension.1);
    for row in 0..image_dimension.1 {
        for column in 0..image_dimension.0 {
            let complex_number = pixel_to_complex_number(
                image_dimension,
                (column, row),
                complex_upper_left,
                complex_bottom_right,
            );
            pixels[row * image_dimension.0 + column] = match escape_time(complex_number, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            }
        }
    }
}

fn run_in_single_thread() {
    let image_dimension = (1000, 750);
    let mut pixels = vec![0; image_dimension.0 * image_dimension.1];
    let complex_upper_left = Complex {
        re: -1.20,
        im: 0.35,
    };
    let complex_bottom_right = Complex { re: -1.0, im: 0.20 };
    render(
        &mut pixels,
        image_dimension,
        complex_upper_left,
        complex_bottom_right,
    );
    write_png("mandelbrot.png", &pixels, image_dimension).expect("Error writing png file");
}