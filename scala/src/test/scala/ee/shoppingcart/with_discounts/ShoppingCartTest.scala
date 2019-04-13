package ee.shoppingcart.with_discounts

import ee.shoppingcart._
import org.specs2.mutable.Specification

class ShoppingCartTest extends Specification {
  "ShoppingCart" should {
    "work for no items" in {
      val shoppingCart = new ShoppingCart
      shoppingCart.contents should beEmpty
      shoppingCart.total() should_== 0
      shoppingCart.salesTax() should_== 0
    }

    "calculate total/sales tax for 5 Dove Soaps" in {
      val shoppingCart = new ShoppingCart()
        .add(DoveSoap(), DoveSoap(), DoveSoap(), DoveSoap(), DoveSoap())
      shoppingCart.contents should_== Seq.fill(5)(DoveSoap())
      shoppingCart.total() should_== 224.94
      shoppingCart.salesTax() should_== 24.99
    }

    "calculate total/sales tax for 5 + 3 Dove Soaps" in {
      val shoppingCart = new ShoppingCart()
        .add(DoveSoap(), DoveSoap(), DoveSoap(), DoveSoap(), DoveSoap())
        .add(DoveSoap(), DoveSoap(), DoveSoap())
      shoppingCart.contents should_== Seq.fill(8)(DoveSoap())
      shoppingCart.total() should_== 359.91
      shoppingCart.salesTax() should_== 39.99
    }

    "calculate total/sales tax for 2 Dove Soaps + 2 Axe Deos" in {
      val shoppingCart = new ShoppingCart()
        .add(DoveSoap(), DoveSoap())
        .add(AxeDeo(), AxeDeo())
      shoppingCart.contents should_== Seq.fill(2)(DoveSoap()) ++ Seq.fill(2)(AxeDeo())
      shoppingCart.total() should_== 314.96
      shoppingCart.salesTax() should_== 35.0
    }

    "apply % off discounts" in {
      val shoppingCart = new ShoppingCart()
        .add(DoveSoap(), AxeSoap())
      // discounted soap
      shoppingCart.total(DiscountRule.soapDiscounts) should_== 70.85
      shoppingCart.salesTax(DiscountRule.soapDiscounts) should_== 7.87
      // discounted soap == no discount!
      shoppingCart.total() should_== 78.73
      shoppingCart.salesTax() should_== 8.75
      shoppingCart.total(DiscountRule.shampooDiscounts) should_== 78.73
      shoppingCart.salesTax(DiscountRule.shampooDiscounts) should_== 8.75
    }

    "apply x for y discount" in {
      // shampoo discount - no discount
      val shoppingCart = new ShoppingCart().add(AxeShampoo())
      shoppingCart.total(DiscountRule.shampooDiscounts) should_== 14.61
      shoppingCart.salesTax(DiscountRule.shampooDiscounts) should_== 1.62
      // + discounted shampoo - no discount
      val shoppingCart2 = shoppingCart.add(AxeShampoo())
      shoppingCart2.total(DiscountRule.shampooDiscounts) should_== 29.23
      shoppingCart2.salesTax(DiscountRule.shampooDiscounts) should_== 3.25
      // + discounted shampoo - have discount!
      val shoppingCart3 = shoppingCart2.add(AxeShampoo())
      shoppingCart3.total(DiscountRule.shampooDiscounts) should_== 29.23
      shoppingCart3.salesTax(DiscountRule.shampooDiscounts) should_== 3.25
    }

    "apply deal discount" in {
      // shampoo discount - no discount
      val shoppingCart = new ShoppingCart().add(AxeShampoo(),AxeDeo())
      shoppingCart.total(DiscountRule.dealDiscounts) should_== 11.25
      shoppingCart.salesTax(DiscountRule.dealDiscounts) should_== 1.25
      // +1 deal
      val shoppingCart2 = shoppingCart.add(AxeShampoo(),AxeDeo())
      shoppingCart2.total(DiscountRule.dealDiscounts) should_== 22.5
      shoppingCart2.salesTax(DiscountRule.dealDiscounts) should_== 2.50
      // +1 non-deal
      val shoppingCart3 = shoppingCart2.add(AxeShampoo())
      shoppingCart3.total(DiscountRule.dealDiscounts) should_== 37.11
      shoppingCart3.salesTax(DiscountRule.dealDiscounts) should_== 4.12
    }
  }
}
