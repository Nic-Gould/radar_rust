fn main() {

    use esp_idf_sys::{self as _}; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
    
    use esp_idf_hal::delay::FreeRtos;
    use esp_idf_hal::peripherals::Peripherals;
    use esp_idf_hal::rmt::{
        FixedLengthSignal, PinState, Pulse, PulseTicks, Receive, RmtReceiveConfig, RmtTransmitConfig,
        RxRmtDriver, TxRmtDriver,
    };
    
    /*It looks like the RMT timer doesn't have the resolution to do 1GHz. 
    Deep in the bowels of the ESP32 hardware documentation, it mentions that the RF module used for the wifi 
    can be configured through software to use different frequencies, but I'm unable to find any way of acheiving
     this in the RF calibration documentation. In the meantime, I'll use the 2.4GHz wifi clock*/
     
    
    fn main() -> anyhow::Result<()> {
        println!("Starting APP!");
    
        let peripherals = Peripherals::take().unwrap();
        let tx_pin = gpio4;
        let rx_pin = gpio2;
    
    //measure system noise when not transmitting
    
    fn calc_noise(){
        modified_confidence_interval = 1.96;
        noise = avg(&sample);
        cutoff = noise + modified_confidence_interval* stdev(sample);
        }
        
    pub struct settings{				// All user configureable settings should go in here.
        samples_per_block: 10;
        carrier_frequency = 2.4e9;		// 2.4GHz;
        chirp_length = 1e-6; 			//1us
        pulse_repitition_time = 1e-3;	//1ms
    }	
        
        let max_range = c * (pulse_repitition_time-pulse_length)/2;
        let min_range = c * pulse_length/2;		// I think it would be fine to remove the divisor as this is rarely going to be a concern in this design and will further minimise near field interference.
        let bits_per_bite = 8; // just for clarity
        let pulse_ticks = chirp_length*carrier_frequency/bits_per_bite;
    
    
    
    
    }
    impl Chirp{                                     //To OOP, or not to OOP, that is the question. Maybe I'll just dump it all into the outer scope and then group it rather than trying to force seperation based on arbitraty structs. 
        fn new(key:u8)->Chirp{
            Chirp{
                
                pulse: bitsfrom(key),
                elsup: pulse.reverse_bits,
            }
        }
    }
    
    struct DataBlock{
        length:i32,
        depth:i32,
        data:<u8>,
        block:<data>,
    }
    impl DataBlock{
    
        fn new (chirp)-> DataBlock {
            DataBlock{
                block = Vec::<Vec<u8>;settings.depth>,
                data = Vec::from(chirp),
                block[0]=data;
            }	
        }
    
    
    
    fn integrate (sample: &Sample){
        data+=sample;
    }
    
    fn filter(cutoff){
        for i in 0..data.len(){
            if data[i] < cutoff{
                data[i] = 0;
            }
        }
    }
    
    fn match_filter(){					// Just a 1d convulution algorithm.
        for j in 0..sample.len(){
            for i in 0..chirp.length{
                sample[j]+=sample[j+i]*eslup[i];
            }
        }
    }
    
    
    
    fn find_peak(){
        // tried a double derivate method but this should be easier, and probably more accurate as the double derivative of the peak 
        //is only zero where the attack and decay are symetrical around the point, which is unlikely due to noise.
        
        let peak=(0,0);	
            for i in 0..sample.len(){
                if sample[i]>peak[1] {
                    peak = (i,sample[1])
            }
        }
    }
    
    
    
        /*
         *********************** SET UP RMT RECEIVER ******************************
         */
        let mut rx = RxRmtDriver::new(
            peripherals.rmt.channel2,
            peripherals.pins.rx_pin,
            &RmtReceiveConfig::new().idle_threshold(700u16),
            250,
        )?;
    
        rx.start().unwrap();
    
        let _ = std::thread::Builder::new()
            .stack_size(10000)
            .spawn(move || loop {
                println!("Rx Loop");
    
                let mut pulses = [(Pulse::zero(), Pulse::zero()); 250];
    
                // See sdkconfig.defaults to determine the tick time value ( default is one tick = 10 milliseconds)
                // Set ticks_to_wait to 0 for non-blocking
                let receive = rx.receive(&mut pulses, 0).unwrap();
    
                if let Receive::Read(length) = receive {
                    let pulses = &pulses[..length];
    
                    for (pulse0, pulse1) in pulses {
                        println!("0={pulse0:?}, 1={pulse1:?}");
                    }
                }
    
                FreeRtos::delay_ms(500);
            });
    
        /*
         *********************** SET UP RMT TRANSMITTER ******************************
         */
    
        // Prepare the tx_config
        // The default uses one memory block or 64 signals and clock divider set to 80 (1us tick)
        let mut tx = TxRmtDriver::new(
            peripherals.rmt.channel0,
            peripherals.pins.tx_pin,
            &RmtTransmitConfig::new(),
        )?;
    
        // Prepare signal pulse signal to be sent.
        let on = Pulse::new(PinState::High, PulseTicks::new(pulse_ticks)?);   		//definately need to check the transmit clock frequenct
        let off = Pulse::new(PinState::Low, PulseTicks::new(pulse_ticks)?);
      
        let _ = std::thread::spawn(move || loop {							//move to different core so rx and tx can run independantly
            println!("Tx Loop");
    
            // Create a sequence from the chirp characters binary value
            let mut signal = FixedLengthSignal::<8>::new();					
            let bump = 0;
            for i in 0..8{
                if chirp>>bump &1 = 1{
                    signal.set(i, &(on)).unwrap();
                }else{
                    signal.set(i, &off)).unwrap();
                }
            }
            
            // Transmit the signal (send sequence)
            
            loop{
                tx.start(signal).unwrap();
                FreeRtos::delay_ms(receiving_time);
            }
        });
    
    }
    }
    
    
    
    
    RADAR.txt
    Displaying RADAR.txt.
}
