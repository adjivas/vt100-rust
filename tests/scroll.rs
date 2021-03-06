extern crate vt100;

mod support;
use support::TestHelpers;

#[test]
fn scroll_regions() {
    let mut screen = vt100::Screen::new(24, 80);
    screen.assert_process(b"\x1b[m\x1b[2J\x1b[H1\r\n2\r\n3\r\n4\r\n5\r\n6\r\n7\r\n8\r\n9\r\n10\r\n11\r\n12\r\n13\r\n14\r\n15\r\n16\r\n17\r\n18\r\n19\r\n20\r\n21\r\n22\r\n23\r\n24");
    assert_eq!(screen.window_contents(0, 0, 23, 79), "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24\n");

    screen.assert_process(b"\x1b[24;50H\n");
    assert_eq!(screen.window_contents(0, 0, 23, 79), "2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24\n\n");

    screen.assert_process(b"\x1b[m\x1b[2J\x1b[H1\r\n2\r\n3\r\n4\r\n5\r\n6\r\n7\r\n8\r\n9\r\n10\r\n11\r\n12\r\n13\r\n14\r\n15\r\n16\r\n17\r\n18\r\n19\r\n20\r\n21\r\n22\r\n23\r\n24");
    screen.assert_process(b"\x1b[10;20r\x1b[20;50H\n");
    assert_eq!(screen.window_contents(0, 0, 23, 79), "1\n2\n3\n4\n5\n6\n7\n8\n9\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n\n21\n22\n23\n24\n");
    assert_eq!(screen.cursor_position(), (19, 49));

    screen.assert_process(b"\x1b[B");
    assert_eq!(screen.cursor_position(), (19, 49));

    screen.assert_process(b"\x1b[20A");
    assert_eq!(screen.cursor_position(), (9, 49));
    screen.assert_process(b"\x1b[1;24r\x1b[m\x1b[2J\x1b[H1\r\n2\r\n3\r\n4\r\n5\r\n6\r\n7\r\n8\r\n9\r\n10\r\n11\r\n12\r\n13\r\n14\r\n15\r\n16\r\n17\r\n18\r\n19\r\n20\r\n21\r\n22\r\n23\r\n24");
    screen.assert_process(b"\x1b[10;20r\x1b[15;50H\x1b[2L");
    assert_eq!(screen.window_contents(0, 0, 23, 79), "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n\n\n15\n16\n17\n18\n21\n22\n23\n24\n");
    screen.assert_process(b"\x1b[10;50H\x1bM");
    assert_eq!(screen.window_contents(0, 0, 23, 79), "1\n2\n3\n4\n5\n6\n7\n8\n9\n\n10\n11\n12\n13\n14\n\n\n15\n16\n17\n21\n22\n23\n24\n");
}
