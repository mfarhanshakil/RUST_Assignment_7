
//------------------------------------------------------------------------------------
//----------------------------------START OF PROGRAM----------------------------------
//------------------------------------------------------------------------------------

// THIS PROGRAM IS DEVELOPED BY FARHAN SHAKIL ON 19-JAN-2020
// BATCH: PIAIC IoT BATCH # 3 ISLAMABAD
// REGISTRATION # PIAIC57331

// THIS PROGRAM IS THE TASK # 2 OF ASSIGNMENT # 7
// "lib.rs" IS CREATED ALONGWITH "main.rs"
// MODULE IS CREATED INSIDE "lib.rs" WITH A SUB-MODULE INSIDE 
// FUNCTION IS DEFINED INSIDE SUB-MODULE AND THEN IT IS CALLED FROM MAIN FUNCTION

//-----------------------------------------------------------------------------------
//------------------------------------MAIN FUNCTION----------------------------------
//-----------------------------------------------------------------------------------

mod lib;
use assig7_task2::piaic::about;

fn main()                                                   // START OF MAIN FUNCTION
{                           
    about::mission();                   // CALL TO FUNCTION "details()" FROM "lib.rs"
    about::programs_offered(); // CALL TO FUNCTION "programs_offered()" FROM "lib.rs"
}                                                             // END OF MAIN FUNCTION

//------------------------------------------------------------------------------------
//-----------------------------------END OF PROGRAM-----------------------------------
//------------------------------------------------------------------------------------