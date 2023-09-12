package at.bumzack.simple

import jakarta.persistence.Entity
import jakarta.persistence.GeneratedValue
import jakarta.persistence.GenerationType
import jakarta.persistence.Id
import java.math.BigDecimal

@Entity(name = "prices")
data class Price(
    @Id @GeneratedValue(strategy = GenerationType.AUTO)
    val id: Long,
    val article_code: Long,
    val price: BigDecimal,
    val currency: String,
)

