
//------------------------------------------------------------------------------------
//----------------------------------START OF PROGRAM----------------------------------
//------------------------------------------------------------------------------------

// THIS PROGRAM IS DEVELOPED BY FARHAN SHAKIL ON 19-JAN-2020
// BATCH: PIAIC IoT BATCH # 3 ISLAMABAD
// REGISTRATION # PIAIC57331

// THIS PROGRAM IS THE TASK # 3 OF ASSIGNMENT # 7
// LIBRARY PACKAGE IS CREATED OTHER THAN BINARY PACKAGE
// MODULE IS CREATED INSIDE LIBRARY PACKAGE WITH A SUB-MODULE INSIDE 
// FUNCTION IS DEFINED INSIDE SUB-MODULE AND THEN IT IS CALLED FROM MAIN FUNCTION
// "Cargo.toml" FILE IS UPDATED WITH PATH OF LIBRARY IN DEPENDENCIES SECTION

//-----------------------------------------------------------------------------------
//------------------------------------MAIN FUNCTION----------------------------------
//-----------------------------------------------------------------------------------

use piaic_lib::piaic::about;                        // DECLARATION TO PATH OF LIBRARY

fn main()                                                   // START OF MAIN FUNCTION
{                           
    about::mission();                                 // CALL TO FUNCTION "details()"
    about::programs_offered();               // CALL TO FUNCTION "programs_offered()"
}                                                             // END OF MAIN FUNCTION

//------------------------------------------------------------------------------------
//-----------------------------------END OF PROGRAM-----------------------------------
//------------------------------------------------------------------------------------