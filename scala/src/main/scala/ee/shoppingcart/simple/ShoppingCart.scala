package ee.shoppingcart.simple

import ee.shoppingcart.Item

import scala.math.BigDecimal.RoundingMode

/**
  * Shopping cart catering for totals and sales tax
  */
case class ShoppingCart(contents: Seq[Item] = Nil) {
  def add(items: Item*): ShoppingCart =
    ShoppingCart(contents ++ items)

  def total: BigDecimal =
    (preTaxTotal + salesTax).setScale(2, RoundingMode.HALF_UP)

  def salesTax: BigDecimal =
    (preTaxTotal * ShoppingCart.salesTax).setScale(2, RoundingMode.HALF_UP)

  private lazy val preTaxTotal: BigDecimal =
    contents.map(_.price).sum
}

object ShoppingCart {
  val salesTax: BigDecimal = BigDecimal(0.125)
}
