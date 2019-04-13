package ee.shoppingcart.with_discounts_and_notifications

import ee.shoppingcart.{AxeDeo, AxeShampoo, AxeSoap, DoveSoap}
import org.specs2.mutable.Specification

class ShoppingCartTest extends Specification {
  "ShoppingCart" should {
    "work for no items" in {
      val shoppingCart = new ShoppingCart2
      shoppingCart.contents should beEmpty
      shoppingCart.total() should_== (0, Nil)
      shoppingCart.salesTax() should_== (0, Nil)
    }

    "calculate total/sales tax for 5 Dove Soaps" in {
      val shoppingCart = new ShoppingCart2()
        .add(DoveSoap(), DoveSoap(), DoveSoap(), DoveSoap(), DoveSoap())
      shoppingCart.contents should_== Seq.fill(5)(DoveSoap())
      shoppingCart.total() should_== (224.94, Nil)
      shoppingCart.salesTax() should_== (24.99, Nil)
    }

    "calculate total/sales tax for 5 + 3 Dove Soaps" in {
      val shoppingCart = new ShoppingCart2()
        .add(DoveSoap(), DoveSoap(), DoveSoap(), DoveSoap(), DoveSoap())
        .add(DoveSoap(), DoveSoap(), DoveSoap())
      shoppingCart.contents should_== Seq.fill(8)(DoveSoap())
      shoppingCart.total() should_== (359.91, Nil)
      shoppingCart.salesTax() should_== (39.99, Nil)
    }

    "calculate total/sales tax for 2 Dove Soaps + 2 Axe Deos" in {
      val shoppingCart = new ShoppingCart2()
        .add(DoveSoap(), DoveSoap())
        .add(AxeDeo(), AxeDeo())
      shoppingCart.contents should_== Seq.fill(2)(DoveSoap()) ++ Seq.fill(2)(AxeDeo())
      shoppingCart.total() should_== (314.96, Nil)
      shoppingCart.salesTax() should_== (35.0, Nil)
    }

    "apply % off discounts" in {
      val shoppingCart = new ShoppingCart2()
        .add(DoveSoap(), AxeSoap())
      // discounted soap
      shoppingCart.total(DiscountRule.soapDiscounts) should_== (70.85, Nil)
      shoppingCart.salesTax(DiscountRule.soapDiscounts) should_== (7.87, Nil)
      // discounted soap == no discount!
      shoppingCart.total() should_== (78.73, Nil)
      shoppingCart.salesTax() should_== (8.75, Nil)
      shoppingCart.total(DiscountRule.shampooDiscounts) should_== (78.73, Nil)
      shoppingCart.salesTax(DiscountRule.shampooDiscounts) should_== (8.75, Nil)
    }

    "apply x for y discount" in {
      // shampoo discount - no discount yet, but notification
      val shoppingCart = new ShoppingCart2().add(AxeShampoo())
      shoppingCart.total(DiscountRule.shampooDiscounts) should_== (14.61, Seq("AxeShampoo: eligible for 3 for the price of 2"))
      shoppingCart.salesTax(DiscountRule.shampooDiscounts) should_== (1.62, Seq("AxeShampoo: eligible for 3 for the price of 2"))
      // + discounted shampoo - no discount, but notification
      val shoppingCart2 = shoppingCart.add(AxeShampoo())
      shoppingCart2.total(DiscountRule.shampooDiscounts) should_== (29.23, Seq("AxeShampoo: eligible for 3 for the price of 2"))
      shoppingCart2.salesTax(DiscountRule.shampooDiscounts) should_== (3.25, Seq("AxeShampoo: eligible for 3 for the price of 2"))
      // + discounted shampoo - have discount!
      val shoppingCart3 = shoppingCart2.add(AxeShampoo())
      shoppingCart3.total(DiscountRule.shampooDiscounts) should_== (29.23, Nil)
      shoppingCart3.salesTax(DiscountRule.shampooDiscounts) should_== (3.25, Nil)
    }
  }
}
