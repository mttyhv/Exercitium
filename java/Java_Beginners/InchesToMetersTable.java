/*
Adapt Try This 1-2 so that it prints a conversion table of inches to meters.
Display 12 feet of conversions, inch by inch. 
Output a blank line every 12 inches. 
(One meter equals approximately 39.37 inches.)
*/
class InchesToMetersTable {
	public static void main(String[] args) {
		double inches, meters;
		int counter;

		counter = 0;
		for(inches = 1; inches <= 144; inches++) {
			meters = inches / 39.37; // convert inches to meters
			System.out.println(inches + " inches is " + meters + " meters.");

			counter++;
			// every 12th line, print a blank line
			if(counter == 12) {
				System.out.println();
				counter = 0; // reset the line counter
			}
		}
	}
}