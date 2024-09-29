use core::panic;
use magnus::{exception::arg_error, function, prelude::*, scan_args::scan_args, Error, RArray, Ruby, Value};
use polyline;
use geo_types::Coord;

fn get_precision(value: Option<i32>) -> Result<u32, Error> {
    match value {
        Some(15..) => Err(Error::new(arg_error(), "precision too high (https://xkcd.com/2170/)")),
        Some(num) => {
            if num < 0 {
                return Err(Error::new(arg_error(), "negative precision doesn't make sense"))
            }
            Ok(num as u32)
        },
        None => Ok(5),
    }
}

fn rb_fast_polylines_decode(args: &[Value]) -> Result<RArray, Error> {
    let args = scan_args::<(Option<String>,), (Option<i32>,), (), (), (), ()>(args)?;

    let (arg_precision,) = args.optional;
    let precision = get_precision(arg_precision)?;

    let polyline = match args.required {
        (Some(p),) => match polyline::decode_polyline(&p, precision) {
            Ok(polyline) => Ok(polyline),
            Err(e) => Err(Error::new(arg_error(), e)),
        },
        (None,) => panic!("no first argument"),
    };

    let array = RArray::new();

    for point in polyline.unwrap().into_points() {
        let inner_rray = RArray::from_vec(vec![point.y(), point.x()]);
        array.push(inner_rray)?
    }
    Ok(array)
}

fn rb_fast_polylines_encode(args: &[Value]) -> Result<String, Error> {
    let args = scan_args::<(Option<RArray>,), (Option<i32>,), (), (), (), ()>(args)?;

    let (points,) = args.required;
    let points = points.unwrap();

    let (precision,) = args.optional;
    let precision = get_precision(precision)?;

    let coordinates = points.each().map(|l| {
        let inner_array = RArray::from_value(l.unwrap());
        let r: [f64; 2] = inner_array.unwrap().to_array().unwrap();
        Coord { y: r[0], x: r[1] }
    });

    match polyline::encode_coordinates(coordinates.into_iter(), precision) {
        Ok(encoded) => Ok(encoded),
        Err(_) => {
            Err(Error::new(arg_error(), "hoge"))
        },
    }
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("FastPolylines")?;
    module.define_singleton_method("decode", function!(rb_fast_polylines_decode, -1))?;
    module.define_singleton_method("encode", function!(rb_fast_polylines_encode, -1))?;
    Ok(())
}
