package at.bumzack.simple

import org.springframework.data.repository.CrudRepository


interface ImageRepository : CrudRepository<Image, Long?> {
//    @Query("SELECT a FROM Articles ORDER BY code LIMIT :pageSize OFFSET :start ")
//    fun getArticlesPaginated(start: Int, pageSize: Int): Iterable<Article?>?
}