CREATE TABLE prices
(
    id     BIGSERIAL PRIMARY KEY not null ,
    article_code BIGINT not null ,
    price NUMERIC(10, 2) not null ,
    currency CHAR(50) not null ,

    CONSTRAINT fk_article_price
        FOREIGN KEY(article_code)
            REFERENCES articles(code)

)
