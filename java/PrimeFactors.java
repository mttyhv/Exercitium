import java.util.Scanner;

public class PrimeFactors {

	public static void main(String[] args) {

		Scanner sc = new Scanner(System.in);
		System.out.print("Insira o número a ser fatorado: ");
		int number = sc.nextInt();

		System.out.print("Fatores primos: ");
		for (int i = 2; i <= number; i++) {
			while (number % i == 0) {
				System.out.print(i + " ");
				number = number / i;
			}
		}
		System.out.println();
		sc.close();
	}
}