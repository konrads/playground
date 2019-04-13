package ee.shoppingcart.with_discounts_and_notifications

import DiscountRule.{DiscountRule, Notice}
import ee.shoppingcart.Item
import ee.shoppingcart.with_discounts.ShoppingCart

import scala.math.BigDecimal.RoundingMode

case class ShoppingCart2(contents: Seq[Item] = Nil) {
  def add(items: Item*): ShoppingCart2 =
    ShoppingCart2(contents ++ items)

  def total(discountRules: Seq[DiscountRule] = Nil): (BigDecimal, Seq[Notice]) = {
    val (total, notices) = preTaxTotal(discountRules)
    val postTaxTotal = (total * (BigDecimal(1) + ShoppingCart.salesTax)).setScale(2, RoundingMode.HALF_UP)
    (postTaxTotal, notices)
  }

  def salesTax(discountRules: Seq[DiscountRule] = Nil): (BigDecimal, Seq[Notice]) = {
    val (total, notices) = preTaxTotal(discountRules)
    val salesTax = (total * ShoppingCart.salesTax).setScale(2, RoundingMode.HALF_UP)
    (salesTax, notices)
  }

  private def preTaxTotal(discountRules: Seq[DiscountRule]): (BigDecimal, Seq[Notice]) = {
    val discounted = discountRules.foldLeft(contents.map((_, None:Option[Notice]))) {
      case (items, rule) =>
        val res = rule(items.map(_._1))
        res
    }
    val total = discounted.map(_._1.price).sum
    val notices = discounted.map(_._2).flatMap(_.toSeq).distinct
    (total, notices)
  }
}

object ShoppingCart2 {
  val salesTax: BigDecimal = BigDecimal(0.125)
}
