package langfeatures

trait Cmd {
  def execute()
}

class Switch {
  private var cmdsSoFar: List[Cmd] = Nil

  def execute(cmd: Cmd): Unit = {
    cmd.execute()
    cmdsSoFar :+= cmd
  }

  def history: List[Cmd] = cmdsSoFar
}

class LightOnCmd extends Cmd {
  override def execute(): Unit = println("on")
}

class LightOffCmd extends Cmd {
  override def execute(): Unit = println("off")
}