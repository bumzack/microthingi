package at.bumzack.simple

import jakarta.persistence.Entity
import jakarta.persistence.GeneratedValue
import jakarta.persistence.GenerationType
import jakarta.persistence.Id

@Entity(name = "articles")
data class Article(
    @Id @GeneratedValue(strategy = GenerationType.AUTO)
    val code: Long
)