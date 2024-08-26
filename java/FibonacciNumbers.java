public class Fibonacci {
	public static void main(String[] args) {
		int limit = 20;
		int previousNumber = 0;
		int currentNumber = 1;
		System.out.print(previousNumber + " " + currentNumber);

		for (int i = 2; i < limit; i++) {
		int nextNumber = previousNumber + currentNumber;
		System.out.print(" " + nextNumber);
		previousNumber = currentNumber;
		currentNumber = nextNumber;
		}
	}
}