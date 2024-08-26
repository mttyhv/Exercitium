import java.util.Scanner;

public class Factorization {

	public static void main(String[] args) {

		Scanner sc = new Scanner(System.in);
		System.out.print("Insira o número a ser fatorado: ");
		int number = sc.nextInt();

		System.out.print("Fatores: ");
		for (int i = 1; i <= number; i++) {
			if (number % i == 0) {
				System.out.print(i + " ");
			}
		}

		sc.close();
	}
}