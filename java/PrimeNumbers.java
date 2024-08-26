import java.util.Scanner;

public class PrimeNumbers {

	public static void main(String[] args) {
		Scanner sc = new Scanner(System.in);
		System.out.print("Insert the limit: ");
		int limit = sc.nextInt();
		sc.close();

		System.out.println("Prime numbers until " + limit + ":");
		for (int i = 2; i <= limit; i++) {
			if (isPrime(i)) {
				System.out.print(i + " ");
			}
		}
		System.out.println();
	}

	public static boolean isPrime(int number) {
		for (int i = 2; i <= Math.sqrt(number); i++) {
			if (number % i == 0) {
				return false;
			}
		}
		return true;
	}
}