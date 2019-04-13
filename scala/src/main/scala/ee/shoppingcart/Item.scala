package ee.shoppingcart

sealed trait Item {
  val price: BigDecimal
  val tags: Seq[String]
}

case class DoveSoap(price: BigDecimal = 39.99, tags: Seq[String] = Seq("Dove Soap", "soap")) extends Item

case class AxeSoap(price: BigDecimal = 29.99, tags: Seq[String] = Seq("Axe Soap", "soap")) extends Item

case class AxeDeo(price: BigDecimal = 99.99, tags: Seq[String] = Seq("Axe Deo", "deodorant", "axe-deodorant-deal")) extends Item

case class AxeShampoo(price: BigDecimal = 12.99, tags: Seq[String] = Seq("Axe Shampoo", "shampoo", "axe-shampoo-deal")) extends Item

case class Deal(price: BigDecimal, tags: Seq[String] = Nil, contents: Seq[Item]) extends Item
