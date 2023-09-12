package at.bumzack.simple

import org.springframework.http.HttpStatus
import org.springframework.http.MediaType
import org.springframework.http.ResponseEntity
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.ResponseBody
import org.springframework.web.bind.annotation.RestController


// @CrossOrigin(maxAge = 3600)
@RestController
class ProductController(
    val articleRepository: ArticleRepository,
    val imageRepository: ImageRepository,
    val priceRepository: PriceRepository,
    val textRepository: TextRepository
) {
    @GetMapping("/article", produces = [MediaType.APPLICATION_JSON_VALUE])
    @ResponseBody
    fun getArticle(): ResponseEntity<List<Article>> {
        val articles = articleRepository.findAll().toList()
        return ResponseEntity(articles, HttpStatus.OK)
    }

    @GetMapping("/image", produces = [MediaType.APPLICATION_JSON_VALUE])
    @ResponseBody
    fun getImages(): ResponseEntity<List<Image>> {
        val articles = imageRepository.findAll().toList()
        return ResponseEntity(articles, HttpStatus.OK)
    }

    @GetMapping("/price", produces = [MediaType.APPLICATION_JSON_VALUE])
    @ResponseBody
    fun getPrice(): ResponseEntity<List<Price>> {
        val articles = priceRepository.findAll().toList()
        return ResponseEntity(articles, HttpStatus.OK)
    }

    @GetMapping("/text", produces = [MediaType.APPLICATION_JSON_VALUE])
    @ResponseBody
    fun getText(): ResponseEntity<List<Text>> {
        val articles = textRepository.findAll().toList()
        return ResponseEntity(articles, HttpStatus.OK)
    }

}
