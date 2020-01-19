// THIS PROGRAM IS DEVELOPED BY FARHAN SHAKIL ON 19-JAN-2020
// BATCH: PIAIC IoT BATCH # 3 ISLAMABAD
// REGISTRATION # PIAIC57331

// THIS PROGRAM IS THE TASK # 1 OF ASSIGNMENT # 7
// MODULE IS CREATED INSIDE "main.rs" WITH A SUB-MODULE INSIDE 
// FUNCTION IS DEFINED INSIDE SUB-MODULE AND THEN IT IS CALLED FROM MAIN FUNCTION

//-----------------------------------------------------------------------------------
//------------------------------------MAIN FUNCTION----------------------------------
//-----------------------------------------------------------------------------------

fn main()                                                   // START OF MAIN FUNCTION
{                           // CALL TO FUNCTION "mission()" INSIDE SUB-MODULE "about"
    piaic::about::mission();
}                                                             // END OF MAIN FUNCTION

//------------------------------------------------------------------------------------
//-------------------------------MODULE AND SUB-MODULE--------------------------------
//------------------------------------------------------------------------------------

mod piaic                                                   // START OF MODULE "piaic"
{   
    pub mod about                        // START OF SUB-MODULE "about" INSIDE "piaic"
    {
        #[derive(Debug)]      // DERIVE ATTRIBUTE FOR STRUCT PRINTING USING fmt::Debug
        #[allow(dead_code)]      // ALLOW ATTRIBUTE TO SUPRESS UNUSED VARIABLE WARNING
        pub struct Piaic                   // STRUCT TO SAVE NAME AND DETAILS OF PIAIC
        {
            piaic_name    : &'static str,        // STRUCT ELEMENT TO STORE PIAIC NAME
            piaic_mission : &'static str,     // STRUCT ELEMENT TO STORE PIAIC MISSION
        }

        pub fn mission()                       // FUNCTION TO DISPLAY NAME AND MISSION
        {
            let piaic_det = Piaic                 // CREATE INSTANCE OF STRUCT "Piaic"
            {
                piaic_name:PiaicAbout::ProgramName.get_details(),
                piaic_mission:PiaicAbout::ProgramMission.get_details(),
            };

            println!("{}" , piaic_det.piaic_name);            // PRINT NAME ON CONSOLE
            println!("{}" , piaic_det.piaic_mission);      // PRINT MISSION ON CONSOLE
        }

        #[derive(Debug)]        // DERIVE ATTRIBUTE FOR ENUM PRINTING USING fmt::Debug
        #[allow(dead_code)]      // ALLOW ATTRIBUTE TO SUPRESS UNUSED VARIABLE WARNING
        pub enum PiaicAbout                       // ENUM TO GENERATE NAME AND MISSION
        {
            ProgramName,
            ProgramMission,
        }
        
        impl PiaicAbout                          // START OF ENUM IMPLEMENTATION BLOCK 
        {                                    // ENUM METHOD TO RETURN NAME AND MISSION
            fn get_details(&self) -> &'static str 
            {
                match self                 // MATCH STATEMENT TO RETURN STRING LITERAL 
                {
                    PiaicAbout::ProgramName   => "\n  PRESIDENTIAL INITIATIVE FOR ART\
                    IFICIAL INTELLIGENCE & COMPUTING (PIAIC)\n",                      
                    PiaicAbout::ProgramMission => "\tThe mission of PIAIC is to \
                    reshape Pakistan by revolutionizing education, research, and \
                    \n\tbusiness by adopting latest, cutting-edge technologies. \
                    Experts are calling this the 4th \n\tindustrial revolution. \
                    We want Pakistan to become a global hub for AI, data science, \
                    cloud \n\tnative computing, edge computing, blockchain, augmented \
                    reality, and internet of things. \n",                                                   
                }                                            // END OF MATCH STATEMENT
            }                                                // END OF METHOD FOR ENUM
        }                                               // END OF IMPLEMENTATION BLOCK
    }                                                     // END OF SUB-MODULE "about"
}                                                             // END OF MODULE "piaic"

//------------------------------------------------------------------------------------
//-----------------------------------END OF PROGRAM-----------------------------------
//------------------------------------------------------------------------------------