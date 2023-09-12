use std::env;
use std::sync::Once;

use bigdecimal::{BigDecimal, FromPrimitive};
use diesel::{
    Connection, ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl,
    SelectableHelper,
};
use dotenvy::dotenv;
use magick_rust::{magick_wand_genesis, MagickWand};
use svg::node::Text;
use svg::Document;

use crate::schema::articles::code;
use crate::schema::articles::dsl::articles;
use crate::table_articles::{Article, NewArticle};
use crate::table_images::{Image, NewImage};
use crate::table_price::{NewPrice, Prices};
use crate::table_texts::{NewText, Texts};

mod schema;
mod table_articles;
mod table_images;
mod table_price;
mod table_texts;

static START: Once = Once::new();

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_article(conn: &mut PgConnection, c: i64) -> Article {
    use crate::schema::articles;

    let new_article = NewArticle { code: c };

    diesel::insert_into(articles::table)
        .values(&new_article)
        .returning(Article::as_returning())
        .get_result(conn)
        .expect("Error saving new article")
}

pub fn create_image(conn: &mut PgConnection, article_code: i64, data: String) -> Image {
    use crate::schema::images;

    let new_image = NewImage {
        article_code,
        image_base64_encoded: data,
    };

    diesel::insert_into(images::table)
        .values(&new_image)
        .returning(Image::as_returning())
        .get_result(conn)
        .expect("Error saving new image")
}

pub fn create_price(
    conn: &mut PgConnection,
    article_code: i64,
    price: BigDecimal,
    currency: String,
) -> Prices {
    use crate::schema::prices;

    let new_image = NewPrice {
        article_code,
        price,
        currency,
    };

    diesel::insert_into(prices::table)
        .values(&new_image)
        .returning(Prices::as_returning())
        .get_result(conn)
        .expect("Error saving new price")
}

pub fn create_text(
    conn: &mut PgConnection,
    article_code: i64,
    article_name: String,
    article_description: String,
    language: String,
) -> Texts {
    use crate::schema::texts;

    let new_text = NewText {
        article_code,
        article_name,
        article_description,
        language,
    };

    diesel::insert_into(texts::table)
        .values(&new_text)
        .returning(Texts::as_returning())
        .get_result(conn)
        .expect("Error saving new texyt")
}

pub fn find_highest_id(conn: &mut PgConnection) -> i64 {
    let article = articles
        .select(Article::as_select())
        .order(code.desc())
        .first(conn)
        .optional(); // This allows for returning an Option<Post>, otherwise it will throw an error

    match article {
        Ok(Some(article)) => {
            println!("Article with code: {}", article.code);
            article.code + 1
        }
        Ok(None) => {
            println!("Unable to find an article ");
            1
        }
        Err(_) => {
            println!("An error occured while fetching article");
            -1
        }
    }
}

fn main() {
    START.call_once(|| {
        magick_wand_genesis();
    });

    let width = 3840;
    let height = 2160;
    let cnt = 1;

    for i in 1..=cnt {
        generate_image(width, height, i);
    }

    let mut conn = establish_connection();

    let c = find_highest_id(&mut conn);
    let res = create_article(&mut conn, c);

    let article_code = res.code;
    println!("inserted article  {:?}", res);

    let image = create_image(&mut conn, article_code, "hallo".to_string());
    println!("inserted image  {:?}", image);

    let p1 = create_price(
        &mut conn,
        article_code,
        BigDecimal::from_f32(23.45).unwrap(),
        "EUR".to_string(),
    );
    let p2 = create_price(
        &mut conn,
        article_code,
        BigDecimal::from_f32(23.45).unwrap(),
        "CHF".to_string(),
    );

    println!("inserted price 1  {:?}", p1);
    println!("inserted price 2  {:?}", p2);

    let t1 = create_text(
        &mut conn,
        article_code,
        "name".to_string(),
        "desciption".to_string(),
        "en".to_string(),
    );
    let t2 = create_text(
        &mut conn,
        article_code,
        "name".to_string(),
        "beschreibung".to_string(),
        "de".to_string(),
    );
    println!("inserted text1  {:?}", t1);
    println!("inserted text2  {:?}", t2);
}

fn generate_image(width: i32, height: i32, i: i32) {
    let rect = svg::node::element::Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", width)
        .set("height", height)
        .set("fill", "lightblue");

    let t = Text::new(format!("article {}", i));
    let txt = svg::node::element::Text::new()
        .set("x", width / 2 - 900)
        .set("y", height / 2)
        .set("font-weight", "bold")
        .set("font-size", 300)
        .add(t);

    let document = Document::new()
        .set("viewBox", (0, 0, width, height))
        .add(rect)
        .add(txt);

    let path = "img";
    let filename = format!("./{}/article_{}", path, i);
    let filename_svg = format!("{}.svg", filename);
    let filename_png = format!("{}.png", filename);
    svg::save(&filename_svg, &document).unwrap();
    convert_to_png(filename_svg, filename_png, width, height);
}

fn convert_to_png(filename_svg: String, filename_png: String, widht: i32, height: i32) {
    let wand = MagickWand::new();
    wand.read_image(&filename_svg).expect("should find it");
    wand.fit(widht as f64 as usize, height as f64 as usize);
    let x = wand.write_image(&filename_png);
    match x {
        Ok(()) => println!("file save ok "),
        Err(e) => println!("file save crashed  {}", e),
    }
}
