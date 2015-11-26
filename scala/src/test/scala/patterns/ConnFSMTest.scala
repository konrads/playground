package patterns

import org.scalatest._

class ConnFSMTest extends FlatSpec with Matchers {

  "ConnFSM" should "transition conn -> disconn -> conn" in {
    val fsm = new ConnFSM

    var attempts = 0
    var connected = false
    while (attempts < 10 && !connected) {
      connected = fsm.connect("joe", "blow")
      attempts += 1
    }

    attempts = 0
    var disconnected = false
    while (attempts < 10 && !disconnected) {
      disconnected = fsm.disconnect()
      attempts += 1
    }

    attempts = 0
    var reconnected = false
    while (attempts < 10 && !reconnected) {
      reconnected = fsm.connect("joey", "blowy")
      attempts += 1
    }

    // validate the trip
    connected should equal (true)
    disconnected should equal (true)
    reconnected should equal (true)
  }

}
