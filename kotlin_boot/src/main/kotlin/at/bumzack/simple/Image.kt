package at.bumzack.simple

import jakarta.persistence.Entity
import jakarta.persistence.GeneratedValue
import jakarta.persistence.GenerationType
import jakarta.persistence.Id

@Entity(name = "images")
data class Image(
    @Id @GeneratedValue(strategy = GenerationType.AUTO)
    val id: Long,
    val article_code: Long,
    val image_base64_encoded: String,
)
