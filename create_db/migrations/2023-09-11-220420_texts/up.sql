CREATE TABLE texts
(
    id     BIGSERIAL PRIMARY KEY,
    article_code BIGINT not null ,
    article_name CHAR(100) not null ,
    article_description CHAR(500) not null  ,
    language CHAR(50) not null ,


    CONSTRAINT fk_article_price
        FOREIGN KEY(article_code)
            REFERENCES articles(code)

)
