object SumOfMultiples {
  def sumOfMultiples(factors: Set[Int], limit: Int): Int =
    factors.flatMap(x => x until limit by x).sum
}

