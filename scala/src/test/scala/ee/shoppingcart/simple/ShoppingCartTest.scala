package ee.shoppingcart.simple

import ee.shoppingcart.{AxeDeo, DoveSoap}
import org.specs2.mutable.Specification

class ShoppingCartTest extends Specification {
  "ShoppingCart" should {
    "work for no items" in {
      val shoppingCart = new ShoppingCart
      shoppingCart.contents should beEmpty
      shoppingCart.total should_== 0
      shoppingCart.salesTax should_== 0
    }

    "calculate total/sales tax for 5 Dove Soaps" in {
      val shoppingCart = new ShoppingCart()
        .add(DoveSoap(), DoveSoap(), DoveSoap(), DoveSoap(), DoveSoap())
      shoppingCart.contents should_== Seq.fill(5)(DoveSoap())
      shoppingCart.total should_== 224.94
      shoppingCart.salesTax should_== 24.99
    }

    "calculate total/sales tax for 5 + 3 Dove Soaps" in {
      val shoppingCart = new ShoppingCart()
        .add(DoveSoap(), DoveSoap(), DoveSoap(), DoveSoap(), DoveSoap())
        .add(DoveSoap(), DoveSoap(), DoveSoap())
      shoppingCart.contents should_== Seq.fill(8)(DoveSoap())
      shoppingCart.total should_== 359.91
      shoppingCart.salesTax should_== 39.99
    }

    "calculate total/sales tax for 2 Dove Soaps + 2 Axe Deos" in {
      val shoppingCart = new ShoppingCart()
        .add(DoveSoap(), DoveSoap())
        .add(AxeDeo(), AxeDeo())
      shoppingCart.contents should_== Seq.fill(2)(DoveSoap()) ++ Seq.fill(2)(AxeDeo())
      shoppingCart.total should_== 314.96
      shoppingCart.salesTax should_== 35.0
    }
  }
}
