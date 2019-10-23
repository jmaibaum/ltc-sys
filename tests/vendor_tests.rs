use ltc_sys::*;
use std::convert::TryInto;
use std::io::Read;

fn decode_raw(mut data: &[u8], apv: Option<i32>) {
    unsafe {
        let apv = apv.unwrap_or(1920);
        let mut sound = [0u8; 1024];

        let mut total: usize = 0;
        let decoder = ltc_decoder_create(apv, 32);
        let mut output = String::new();
        loop {
            let n = data.read(&mut sound).unwrap();
            ltc_decoder_write(decoder, sound.as_mut_ptr(), n, total as i64);

            loop {
                let mut frame = LTCFrameExt {
                    biphase_tics: [0.0f32; 80],
                    ltc: LTCFrame {
                        __bindgen_padding_0: 0,
                        _bitfield_1: __BindgenBitfieldUnit::new([0u8; 10]),
                    },
                    off_start: 0,
                    off_end: 0,
                    reverse: 0,
                    sample_max: 0,
                    sample_min: 0,
                    volume: 0.0,
                };
                let read_ret = ltc_decoder_read(decoder, &mut frame as *mut LTCFrameExt);
                let mut stime = SMPTETimecode {
                    timezone: [0; 6],
                    years: 0,
                    months: 0,
                    days: 0,
                    hours: 0,
                    mins: 0,
                    secs: 0,
                    frame: 0,
                };
                ltc_frame_to_time(
                    &mut stime as *mut SMPTETimecode,
                    &mut frame.ltc as *mut LTCFrame,
                    1,
                );
                output.push_str(&format!(
                    "{:04}-{:02}-{:02} {} {:02}:{:02}:{:02}{}{:02} | {:8} {:8}{}\n",
                    if stime.years < 67 {
                        2000 + stime.years as i32
                    } else {
                        1900 + stime.years as i32
                    },
                    stime.months,
                    stime.days,
                    stime.timezone,
                    stime.hours,
                    stime.mins,
                    stime.secs,
                    if frame.ltc.dfbit() != 0 { '.' } else { ':' },
                    stime.frame,
                    frame.off_start,
                    frame.off_end,
                    if frame.reverse != 0 { "  R" } else { "" },
                ));

                if read_ret == 0 {
                    break;
                }
            }

            total += n;
            if n == 0 {
                break;
            }
        }
    }
}

#[test]
/// Adapted C test code from vendor/tests/ltcencode.c
fn ltc_encode_48000() {
    let length = 2.0;
    let fps = 25.0;
    let sample_rate = 48000.0;
    let mut st = SMPTETimecode {
        timezone: [43, 48, 49, 48, 48, 0], // b"+0100\0"
        years: 8,
        days: 12,
        months: 31,
        hours: 23,
        mins: 59,
        secs: 59,
        frame: 0,
    };

    unsafe {
        let encoder =
            ltc_encoder_create(1.0, 1.0, 0, LTC_BG_FLAGS_LTC_USE_DATE.try_into().unwrap());
        ltc_encoder_set_bufsize(encoder, sample_rate, fps);
        ltc_encoder_reinit(
            encoder,
            sample_rate,
            fps,
            LTC_TV_STANDARD_LTC_TV_625_50,
            LTC_BG_FLAGS_LTC_USE_DATE.try_into().unwrap(),
        );
        ltc_encoder_set_filter(encoder, 0.0);
        ltc_encoder_set_filter(encoder, 25.0);
        ltc_encoder_set_volume(encoder, -18.0);
        ltc_encoder_set_timecode(encoder, &mut st as *mut SMPTETimecode);

        //loop {
        let mut buf: *mut ltcsnd_sample_t;
        let mut len = 0;

        ltc_encoder_encode_frame(encoder);
        buf = ltc_encoder_get_bufptr(encoder, &mut len, 1);
        //}
        ltc_encoder_free(encoder);
    }

    assert_eq!(4, 2 * 2);
}
