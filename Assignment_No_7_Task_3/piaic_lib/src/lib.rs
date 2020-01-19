
//------------------------------------------------------------------------------------
//-------------------------------START OF LIBRARY FILE--------------------------------
//------------------------------------------------------------------------------------

// THIS LIRARY FILE IS DEVELOPED BY FARHAN SHAKIL ON 19-JAN-2020
// BATCH: PIAIC IoT BATCH # 3 ISLAMABAD
// REGISTRATION # PIAIC57331

// THIS LIBRARY IS THE TASK # 3 OF ASSIGNMENT # 7
// THIS IS LIBRARY PACKAGE FOR PIAIC DATA DISPLAY
// MODULE IS CREATED INSIDE "lib.rs" WITH A SUB-MODULE INSIDE 
// FUNCTION ARE DEFINED INSIDE SUB-MODULE AND CALLED FROM "main.rs"
// ONE FUNCTION DISPLAYS DETAILS AND OTHER PROGRAMS OFFERED

//------------------------------------------------------------------------------------
//-------------------------------MODULE AND SUB-MODULE--------------------------------
//------------------------------------------------------------------------------------

pub mod piaic                                               // START OF MODULE "piaic"
{   
    pub mod about                        // START OF SUB MODULE "about" INSIDE "piaic"
    {
        #[allow(dead_code)]      // ALLOW ATTRIBUTE TO SUPRESS UNUSED FUNCTION WARNING
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

        #[allow(dead_code)]      // ALLOW ATTRIBUTE TO SUPRESS UNUSED FUNCTION WARNING
        pub fn programs_offered()              // FUNCTION TO DISPLAY PROGRAMS OFFERED
        {                                          // VECTOR TO STORE PROGRAMS OFFERED
            let prog_det = [PiaicAbout::AIC.get_details() , 
            PiaicAbout::BCC.get_details() , PiaicAbout::CNC.get_details() ,
            PiaicAbout::IoT.get_details() , PiaicAbout::QMC.get_details() ,
            PiaicAbout::FGS.get_details()];
              
            println!("\n  Below are the Programs Offered by PIAIC: \n");
                                            // DISPLAY PROGRAMS OFFERED USING FOR LOOP
            for lp_var in 0..prog_det.len()
            {
                println!("\t {}) {}" , lp_var + 1 , prog_det[lp_var]); 
        
            }
            println!("");
        }

        #[derive(Debug)]      // DERIVE ATTRIBUTE FOR STRUCT PRINTING USING fmt::Debug
        #[allow(dead_code)]      // ALLOW ATTRIBUTE TO SUPRESS UNUSED VARIABLE WARNING
        pub struct Piaic                   // STRUCT TO SAVE NAME AND DETAILS OF PIAIC
        {
            piaic_name    : &'static str,        // STRUCT ELEMENT TO STORE PIAIC NAME
            piaic_mission : &'static str,     // STRUCT ELEMENT TO STORE PIAIC MISSION
        }

        #[derive(Debug)]        // DERIVE ATTRIBUTE FOR ENUM PRINTING USING fmt::Debug
        #[allow(dead_code)]      // ALLOW ATTRIBUTE TO SUPRESS UNUSED VARIABLE WARNING
        pub enum PiaicAbout  // ENUM TO GENERATE NAME AND MISSION AND PROGRAMS OFFERED
        {
            ProgramName,
            ProgramMission,
            AIC,
            BCC,
            CNC,
            IoT,
            QMC,
            FGS,
        }
        
        impl PiaicAbout                          // START OF ENUM IMPLEMENTATION BLOCK 
        {                                    // ENUM METHOD TO RETURN NAME AND MISSION
            #[allow(dead_code)]      // ALLOW ATTRIBUTE TO SUPRESS UNUSED VARIABLE WARNING
            fn get_details(&self) -> &'static str 
            {
                match self                 // MATCH STATEMENT TO RETURN STRING LITERAL 
                {
                    PiaicAbout::ProgramName   => "\n  PRESIDENTIAL INITIATIVE FOR \
                    ARTIFICIAL INTELLIGENCE & COMPUTING (PIAIC)\n",                      
                    PiaicAbout::ProgramMission => "\tThe mission of PIAIC is to \
                    reshape Pakistan by revolutionizing education, research, and \
                    \n\tbusiness by adopting latest, cutting-edge technologies. \
                    Experts are calling this the 4th \n\tindustrial revolution. \
                    We want Pakistan to become a global hub for AI, data science, \
                    cloud \n\tnative computing, edge computing, blockchain, \
                    augmented reality, and internet of things. \n",     
                    PiaicAbout::AIC => "ARTIFICIAL INTELLIGENCE",
                    PiaicAbout::BCC => "BLOCKCHAIN SPECIALIST",
                    PiaicAbout::CNC => "CLOUD NATIVE AND MOBILE WEB COMPUTING SPECIAL\
                    IST",
                    PiaicAbout::IoT => "INTERNET OF THINGS AND AI SPECIALIST",
                    PiaicAbout::QMC => "QUANTUM COMPUTING",
                    PiaicAbout::FGS => "5G AND SOFTWARE DEFINED NETWORKING (SDN/NFV/O\
                    rchestration)",                                              
                }                                            // END OF MATCH STATEMENT
            }                                                // END OF METHOD FOR ENUM
        }                                               // END OF IMPLEMENTATION BLOCK
    }                                                         // END OF MODULE "about"
}                                                             // END OF MODULE "piaic"

//------------------------------------------------------------------------------------
//-----------------------------------END OF LIBRARY FILE------------------------------
//------------------------------------------------------------------------------------