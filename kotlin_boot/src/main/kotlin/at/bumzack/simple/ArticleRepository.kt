package at.bumzack.simple

import jakarta.persistence.Table
import org.springframework.data.repository.CrudRepository


interface ArticleRepository : CrudRepository<Article, Long?> {
//    @Query("SELECT a FROM Articles ORDER BY code LIMIT :pageSize OFFSET :start ")
//    fun getArticlesPaginated(start: Int, pageSize: Int): Iterable<Article?>?
}