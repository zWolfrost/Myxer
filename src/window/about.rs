/*!
 * Contains the About window.
 */

use gtk::prelude::*;


/**
 * Creates and runs the About popup window,
 * which contains information about the app, license, and Auri.
 */

pub fn about() {
	let about = gtk::AboutDialog::new();
	about.set_logo_icon_name(Some("multimedia-volume-control"));
	about.set_program_name("Myxer");
	about.set_version(Some("1.3.0"));
	about.set_comments(Some("A modern Volume Mixer for PulseAudio."));
	about.set_website(Some("https://github.com/Aurailus/Myxer"));
	about.set_copyright(Some("© 2021 Auri Collings"));
	about.set_license_type(gtk::License::Gpl30);
	about.add_credit_section("Created by", &[ "Auri Collings" ]);
	about.add_credit_section("With the help of", &[ "zWolfrost" ]);
	about.add_credit_section("libpulse-binding by", &[ "Lyndon Brown" ]);

	about.connect_response(|about, _| about.close());
	about.run();
}
