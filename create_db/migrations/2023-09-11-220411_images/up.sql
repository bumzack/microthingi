CREATE TABLE images
(
    id     BIGSERIAL PRIMARY KEY not null ,
    article_code BIGINT not null ,
    image_base64_encoded TEXT not null ,

    CONSTRAINT fk_article_img
        FOREIGN KEY(article_code)
            REFERENCES articles(code)

)