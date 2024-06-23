pub fn opening_book(position: &str) -> Vec<&str> {
    match position {
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -" => {
            vec!["e2e4", "g1f3", "d2d4", "c2c4"]
        }
        "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq -" => vec!["e7e5", "c7c5"],
        "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq -" => vec!["g1f3"],
        "rnbqkbnr/pppp1ppp/8/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq -" => vec!["b8c6", "g8f6"],
        "r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq -" => vec!["f1c4", "f1b5"],
        "r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq -" => vec!["f8c5", "g8f6"],
        "r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq -" => {
            vec!["c2c3", "e1g1", "d2d3"]
        }
        "r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/2P2N2/PP1P1PPP/RNBQK2R b KQkq -" => vec!["g8f6"],
        "r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B1P3/2P2N2/PP1P1PPP/RNBQK2R w KQkq -" => {
            vec!["d2d3", "d2d4", "b2b4"]
        }
        "r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B1P3/2PP1N2/PP3PPP/RNBQK2R b KQkq -" => {
            vec!["d7d6", "e8g8", "a7a5"]
        }
        "r1bqk2r/ppp2ppp/2np1n2/2b1p3/2B1P3/2PP1N2/PP3PPP/RNBQK2R w KQkq -" => {
            vec!["e1g1", "c4b3", "b2b4", "b1d2", "h2h3"]
        }
        "r1bqk2r/ppp2ppp/2np1n2/2b1p3/2B1P3/2PP1N2/PP3PPP/RNBQ1RK1 b kq -" => {
            vec!["e8g8", "a7a5", "c5b6"]
        }
        "r1bq1rk1/ppp2ppp/2np1n2/2b1p3/2B1P3/2PP1N2/PP3PPP/RNBQ1RK1 w - -" => {
            vec!["h2h3", "f1e1", "a2a4", "b1d2", "b2b4"]
        }
        "r1bq1rk1/ppp2ppp/2np1n2/2b1p3/2B1P3/2PP1N1P/PP3PP1/RNBQ1RK1 b - -" => vec!["a7a5", "a7a6"],
        "r1bq1rk1/1pp2ppp/2np1n2/p1b1p3/2B1P3/2PP1N1P/PP3PP1/RNBQ1RK1 w - -" => {
            vec!["f1e1", "b1d2", "a2a4", "c4b5"]
        }
        "r1bq1rk1/1pp2ppp/2np1n2/p1b1p3/2B1P3/2PP1N1P/PP3PP1/RNBQR1K1 b - -" => {
            vec!["c8e6", "h7h6", "c5a7"]
        }
        "r1bq1rk1/ppp2ppp/2np1n2/2b1p3/2B1P3/2PP1N2/PP1N1PPP/R1BQ1RK1 b - -" => vec!["a7a5"],
        "r1bq1rk1/1pp2ppp/2np1n2/p1b1p3/2B1P3/2PP1N2/PP1N1PPP/R1BQ1RK1 w - -" => {
            vec!["f1e1", "h2h3", "a2a4"]
        }
        "r1bq1rk1/1pp2ppp/2np1n2/p1b1p3/2B1P3/2PP1N2/PP1N1PPP/R1BQR1K1 b - -" => {
            vec!["c5b6", "h7h6", "c5a7", "c8e6"]
        }
        "r1bq1rk1/1pp2ppp/2np1n2/p1b1p3/P1B1P3/2PP1N2/1P1N1PPP/R1BQ1RK1 b - -" => {
            vec!["c5a7", "h7h6"]
        }
        "r1bq1rk1/ppp2ppp/2np1n2/2b1p3/1PB1P3/2PP1N2/P4PPP/RNBQ1RK1 b - -" => vec!["c5b6"],
        "r1bqk2r/1pp2ppp/2np1n2/p1b1p3/2B1P3/2PP1N2/PP3PPP/RNBQ1RK1 w kq -" => {
            vec!["f1e1", "a2a4", "b1d2", "h2h3", "c4b3"]
        }
        "r1bqk2r/1pp2ppp/2np1n2/p1b1p3/P1B1P3/2PP1N2/1P3PPP/RNBQ1RK1 b kq -" => vec!["e8g8"],
        "r1bq1rk1/1pp2ppp/2np1n2/p1b1p3/P1B1P3/2PP1N2/1P3PPP/RNBQ1RK1 w - -" => {
            vec!["b1d2", "f1e1", "h2h3", "c1g5"]
        }
        "r1bqk2r/1pp2ppp/2np1n2/p1b1p3/2B1P3/2PP1N1P/PP3PP1/RNBQ1RK1 b kq -" => {
            vec!["h7h6", "e8g8"]
        }
        "r1bqk2r/1pp2pp1/2np1n1p/p1b1p3/2B1P3/2PP1N1P/PP3PP1/RNBQ1RK1 w kq -" => vec!["b1d2"],
        "r1bqk2r/1pp2pp1/2np1n1p/p1b1p3/2B1P3/2PP1N1P/PP1N1PP1/R1BQ1RK1 b kq -" => vec!["e8g8"],
        "r1bqk2r/ppp2pp1/1bnp1n1p/4p3/2B1P3/2PP1N1P/PP3PP1/RNBQ1RK1 w kq -" => vec!["b2b4", "b1d2"],
        "r1bqk2r/ppp2ppp/2np1n2/2b1p3/2B1P3/2PP1N1P/PP3PP1/RNBQK2R b KQkq -" => {
            vec!["h7h6", "a7a5", "e8g8"]
        }
        "r1bqk2r/ppp2pp1/2np1n1p/2b1p3/2B1P3/2PP1N1P/PP3PP1/RNBQK2R w KQkq -" => vec!["e1g1"],
        "r1bqk2r/ppp2pp1/2np1n1p/2b1p3/2B1P3/2PP1N1P/PP3PP1/RNBQ1RK1 b kq -" => {
            vec!["e8g8", "a7a5", "c5b6"]
        }
        "r1bq1rk1/ppp2pp1/2np1n1p/2b1p3/2B1P3/2PP1N1P/PP3PP1/RNBQ1RK1 w - -" => {
            vec!["b2b4", "f1e1"]
        }
        "r1bq1rk1/ppp2pp1/2np1n1p/2b1p3/2B1P3/2PP1N1P/PP3PP1/RNBQR1K1 b - -" => {
            vec!["a7a5", "c5b6"]
        }
        "r1bqk2r/1pp2ppp/2np1n2/p1b1p3/2B1P3/2PP1N1P/PP3PP1/RNBQK2R w KQkq -" => vec!["e1g1"],
        "r1bq1rk1/ppp2ppp/2np1n2/2b1p3/2B1P3/2PP1N1P/PP3PP1/RNBQK2R w KQ -" => vec!["e1g1"],
        "r1bq1rk1/pppp1ppp/2n2n2/2b1p3/2B1P3/2PP1N2/PP3PPP/RNBQ1RK1 b - -" => vec!["c5b6", "d7d6"],
        "r1bq1rk1/pppp1ppp/2n2n2/2b1p3/P1B1P3/2PP1N2/1P3PPP/RNBQK2R b KQ -" => vec!["a7a6", "a7a5"],
        "r1bq1rk1/1ppp1ppp/2n2n2/p1b1p3/P1B1P3/2PP1N2/1P3PPP/RNBQK2R w KQ -" => vec!["e1g1"],
        "r1bq1rk1/1ppp1ppp/2n2n2/p1b1p3/P1B1P3/2PP1N2/1P3PPP/RNBQ1RK1 b - -" => vec!["d7d6"],
        "r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQ1RK1 b kq -" => vec!["g8f6"],
        "r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B1P3/3P1N2/PPP2PPP/RNBQ1RK1 b kq -" => vec!["d7d6"],
        "r1bqk2r/ppp2ppp/2np1n2/2b1p3/2B1P3/3P1N2/PPP2PPP/RNBQ1RK1 w kq -" => vec!["c2c3"],
        "r1bqk2r/ppp2ppp/2np1n2/2b1p3/2B1P3/2NP1N2/PPP2PPP/R1BQ1RK1 b kq -" => vec!["a7a5", "c6a5"],
        "r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R b KQkq -" => vec!["g8f6"],
        "r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R w KQkq -" => {
            vec!["a2a4", "b1c3", "e1g1", "c2c3", "h2h3", "b1d2"]
        }
        "r1bqk2r/pppp1ppp/2n2n2/2b1p3/P1B1P3/3P1N2/1PP2PPP/RNBQK2R b KQkq -" => {
            vec!["d7d6", "a7a6"]
        }
        "r1bqk2r/ppp2ppp/2np1n2/2b1p3/P1B1P3/3P1N2/1PP2PPP/RNBQK2R w KQkq -" => vec!["e1g1"],
        "r1bqk2r/ppp2ppp/2np1n2/2b1p3/P1B1P3/3P1N2/1PP2PPP/RNBQ1RK1 b kq -" => vec!["a7a5"],
        "r1bqk2r/1pp2ppp/2np1n2/p1b1p3/P1B1P3/3P1N2/1PP2PPP/RNBQ1RK1 w kq -" => vec!["c2c3"],
        "r1bqk2r/1ppp1ppp/p1n2n2/2b1p3/P1B1P3/3P1N2/1PP2PPP/RNBQK2R w KQkq -" => vec!["e1g1"],
        "r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B1P3/2NP1N2/PPP2PPP/R1BQK2R b KQkq -" => {
            vec!["d7d6", "h7h6"]
        }
        "r1bqk2r/ppp2ppp/2np1n2/2b1p3/2B1P3/2NP1N2/PPP2PPP/R1BQK2R w KQkq -" => {
            vec!["c3a4", "e1g1"]
        }
        "r1bqk2r/ppp2ppp/2np1n2/2b1p3/N1B1P3/3P1N2/PPP2PPP/R1BQK2R b KQkq -" => vec!["c5b6"],
        "r1bqk2r/ppp2ppp/1bnp1n2/4p3/N1B1P3/3P1N2/PPP2PPP/R1BQK2R w KQkq -" => vec!["a2a3"],
        "r1bqk2r/ppp2pp1/1bnp1n1p/4p3/N1B1P3/P2P1N2/1PP2PPP/R1BQK2R w KQkq -" => {
            vec!["b2b4", "h2h3"]
        }
        "r1bqk2r/pppp1pp1/2n2n1p/2b1p3/2B1P3/2NP1N2/PPP2PPP/R1BQK2R w KQkq -" => {
            vec!["h2h3", "e1g1"]
        }
        "r1bqk2r/pppp1pp1/2n2n1p/2b1p3/2B1P3/2NP1N1P/PPP2PP1/R1BQK2R b KQkq -" => {
            vec!["a7a6", "d7d6"]
        }
        "r1bqk2r/1ppp1pp1/p1n2n1p/2b1p3/2B1P3/2NP1N1P/PPP2PP1/R1BQK2R w KQkq -" => {
            vec!["a2a4", "e1g1"]
        }
        "r1bqk2r/1pp2pp1/p1np1n1p/2b1p3/P1B1P3/2NP1N1P/1PP2PP1/R1BQ1RK1 b kq -" => {
            vec!["e8g8", "c8e6"]
        }
        "r1bqk2r/1ppp1pp1/p1n2n1p/2b1p3/2B1P3/2NP1N1P/PPP2PP1/R1BQ1RK1 b kq -" => {
            vec!["d7d6", "e8g8"]
        }
        "r1bq1rk1/1ppp1pp1/p1n2n1p/2b1p3/2B1P3/2NP1N1P/PPP2PP1/R1BQ1RK1 w - -" => {
            vec!["a2a3", "a2a4"]
        }
        "r1bq1rk1/1ppp1pp1/p1n2n1p/2b1p3/2B1P3/P1NP1N1P/1PP2PP1/R1BQ1RK1 b - -" => vec!["d7d6"],
        "r1bq1rk1/1ppp1pp1/p1n2n1p/2b1p3/P1B1P3/2NP1N1P/1PP2PP1/R1BQ1RK1 b - -" => vec!["d7d6"],
        "r1bqk2r/pppp1pp1/2n2n1p/2b1p3/2B1P3/2NP1N2/PPP2PPP/R1BQ1RK1 b kq -" => vec!["e8g8"],
        "r1bq1rk1/pppp1pp1/2n2n1p/2b1p3/2B1P3/2NP1N2/PPP2PPP/R1BQ1RK1 w - -" => {
            vec!["c3a4", "h2h3"]
        }
        "r1bq1rk1/pppp1pp1/2n2n1p/2b1p3/N1B1P3/3P1N2/PPP2PPP/R1BQ1RK1 b - -" => {
            vec!["c5b6", "c5e7"]
        }
        "r1bq1rk1/pppp1pp1/1bn2n1p/4p3/N1B1P3/3P1N2/PPP2PPP/R1BQ1RK1 w - -" => vec!["h2h3"],
        "r1bq1rk1/pppp1pp1/1bn2n1p/4p3/N1B1P3/3P1N1P/PPP2PP1/R1BQ1RK1 b - -" => vec!["d7d6"],
        "r1bq1rk1/ppppbpp1/2n2n1p/4p3/N1B1P3/3P1N2/PPP2PPP/R1BQ1RK1 w - -" => vec!["a4c3"],
        "r1bq1rk1/ppppbpp1/2n2n1p/4p3/2B1P3/2NP1N2/PPP2PPP/R1BQ1RK1 b - -" => vec!["e7c5"],
        "r1bq1rk1/pppp1pp1/2n2n1p/2b1p3/2B1P3/2NP1N1P/PPP2PP1/R1BQ1RK1 b - -" => {
            vec!["d7d6", "a7a6"]
        }
        "r1bq1rk1/ppp2pp1/2np1n1p/2b1p3/2B1P3/2NP1N1P/PPP2PP1/R1BQ1RK1 w - -" => vec!["a2a3"],
        "r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B1P3/3P1N1P/PPP2PP1/RNBQK2R b KQkq -" => vec!["e8g8"],
        "r1bq1rk1/pppp1ppp/2n2n2/2b1p3/2B1P3/3P1N1P/PPP2PP1/RNBQK2R w KQ -" => vec!["e1g1"],
        "r1bq1rk1/pppp1ppp/2n2n2/2b1p3/2B1P3/3P1N1P/PPP2PP1/RNBQ1RK1 b - -" => vec!["d7d6"],
        "r1bq1rk1/ppp2ppp/2np1n2/2b1p3/2B1P3/3P1N1P/PPP2PP1/RNBQ1RK1 w - -" => vec!["c2c3"],
        "r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B1P3/3P1N2/PPPN1PPP/R1BQK2R b KQkq -" => vec!["e8g8"],
        "r1bq1rk1/pppp1ppp/2n2n2/2b1p3/2B1P3/3P1N2/PPPN1PPP/R1BQK2R w KQ -" => vec!["e1g1"],
        "r1bq1rk1/pppp1ppp/2n2n2/2b1p3/2B1P3/3P1N2/PPPN1PPP/R1BQ1RK1 b - -" => vec!["d7d6"],
        "r1bq1rk1/ppp2ppp/2np1n2/2b1p3/2B1P3/3P1N2/PPPN1PPP/R1BQ1RK1 w - -" => vec!["c2c3", "a2a4"],
        "r1bq1rk1/ppp2ppp/2np1n2/2b1p3/P1B1P3/3P1N2/1PPN1PPP/R1BQ1RK1 b - -" => vec!["a7a5"],
        "r1bq1rk1/1pp2ppp/2np1n2/p1b1p3/P1B1P3/3P1N2/1PPN1PPP/R1BQ1RK1 w - -" => vec!["c2c3"],
        "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq -" => vec!["d2d3"],
        "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R b KQkq -" => vec!["f8c5"],
        "r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq -" => vec!["a7a6", "g8f6"],
        "r1bqkbnr/1ppp1ppp/p1n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq -" => vec!["b5a4"],
        "r1bqkbnr/1ppp1ppp/p1n5/4p3/B3P3/5N2/PPPP1PPP/RNBQK2R b KQkq -" => vec!["g8f6", "f8e7"],
        "r1bqkb1r/1ppp1ppp/p1n2n2/4p3/B3P3/5N2/PPPP1PPP/RNBQK2R w KQkq -" => vec!["d2d3", "e1g1"],
        "r1bqkb1r/1ppp1ppp/p1n2n2/4p3/B3P3/3P1N2/PPP2PPP/RNBQK2R b KQkq -" => vec!["f8c5"],
        "r1bqk2r/1ppp1ppp/p1n2n2/2b1p3/B3P3/3P1N2/PPP2PPP/RNBQK2R w KQkq -" => vec!["a4b3"],
        "r1bqkb1r/1ppp1ppp/p1n2n2/4p3/B3P3/5N2/PPPP1PPP/RNBQ1RK1 b kq -" => vec!["f8e7"],
        "r1bqk2r/1pppbppp/p1n2n2/4p3/B3P3/5N2/PPPP1PPP/RNBQ1RK1 w kq -" => vec!["f1e1", "d2d3"],
        "r1bqk2r/1pppbppp/p1n2n2/4p3/B3P3/5N2/PPPP1PPP/RNBQR1K1 b kq -" => vec!["b7b5"],
        "r1bqk2r/2ppbppp/p1n2n2/1p2p3/B3P3/5N2/PPPP1PPP/RNBQR1K1 w kq -" => vec!["a4b3"],
        "r1bqk2r/2ppbppp/p1n2n2/1p2p3/4P3/1B3N2/PPPP1PPP/RNBQR1K1 b kq -" => vec!["e8g8"],
        "r1bq1rk1/2ppbppp/p1n2n2/1p2p3/4P3/1B3N2/PPPP1PPP/RNBQR1K1 w - -" => {
            vec!["c2c3", "d2d3", "a2a4", "h2h3"]
        }
        "r1bq1rk1/2ppbppp/p1n2n2/1p2p3/4P3/1BP2N2/PP1P1PPP/RNBQR1K1 b - -" => vec!["d7d5", "d7d6"],
        "r1bq1rk1/5ppp/p1pb4/1p1n4/8/1BP5/PP1P1PPP/RNBQR1K1 w - -" => vec!["d2d3", "g2g3", "d2d4"],
        "r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq -" => vec!["d2d3", "e1g1"],
        "r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/3P1N2/PPP2PPP/RNBQK2R b KQkq -" => vec!["f8c5"],
        "r1bqk2r/pppp1ppp/2n2n2/1Bb1p3/4P3/3P1N2/PPP2PPP/RNBQK2R w KQkq -" => {
            vec!["b5c6", "b1d2", "b5a4", "c2c3"]
        }
        "r1bqk2r/pppp1ppp/2B2n2/2b1p3/4P3/3P1N2/PPP2PPP/RNBQK2R b KQkq -" => vec!["d7c6"],
        "r1bqk2r/ppp2ppp/2p2n2/2b1p3/4P3/3P1N2/PPP2PPP/RNBQK2R w KQkq -" => vec!["e1g1"],
        "r1bqk2r/ppp2ppp/2p2n2/2b1p3/4P3/3P1N2/PPP2PPP/RNBQ1RK1 b kq -" => vec!["f6d7"],
        "r1bq1rk1/pppn1ppp/2p5/2b1p3/4P3/3P1N2/PPPN1PPP/R1BQ1RK1 w - -" => vec!["d2c4"],
        "r1bq1rk1/pppn1ppp/2p5/2b1p3/2N1P3/3P1N2/PPP2PPP/R1BQ1RK1 b - -" => vec!["f8e8", "f7f6"],
        "r1bqr1k1/pppn1ppp/2p5/2b1p3/2N1P3/3P1N2/PPP2PPP/R1BQ1RK1 w - -" => {
            vec!["g1h1", "a2a4", "c1e3", "c1d2"]
        }
        "r1bqk2r/pppp1ppp/2n2n2/1Bb1p3/4P3/3P1N2/PPPN1PPP/R1BQK2R b KQkq -" => vec!["e8g8"],
        "r1bq1rk1/pppp1ppp/2n2n2/1Bb1p3/4P3/3P1N2/PPPN1PPP/R1BQK2R w KQ -" => vec!["b5c6"],
        "r1bq1rk1/pppp1ppp/2B2n2/2b1p3/4P3/3P1N2/PPPN1PPP/R1BQK2R b KQ -" => vec!["d7c6"],
        "r1bqkb1r/pppp1ppp/2n5/1B2p3/4n3/5N2/PPPP1PPP/RNBQ1RK1 w kq -" => vec!["d2d4", "f1e1"],
        "r1bk1b1r/ppp2ppp/2p5/4Pn2/8/5N2/PPP2PPP/RNB2RK1 w - -" => vec!["b1c3", "h2h3"],
        "r1bqkbnr/pppp1ppp/2n5/4p3/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq -" => vec!["e5d4"],
        "r1bqkbnr/pppp1ppp/2n5/8/3pP3/5N2/PPP2PPP/RNBQKB1R w KQkq -" => vec!["f3d4"],
        "r1bqkbnr/pppp1ppp/2n5/8/3NP3/8/PPP2PPP/RNBQKB1R b KQkq -" => vec!["f8c5", "g8f6"],
        "r1bqkb1r/pppp1ppp/2n2n2/8/3NP3/8/PPP2PPP/RNBQKB1R w KQkq -" => vec!["d4c6"],
        "r1bqkb1r/pppp1ppp/2N2n2/8/4P3/8/PPP2PPP/RNBQKB1R b KQkq -" => vec!["b7c6"],
        "r1bqkb1r/p1pp1ppp/2p2n2/8/4P3/8/PPP2PPP/RNBQKB1R w KQkq -" => vec!["f1d3"],
        "r1bqkb1r/p1pp1ppp/2p2n2/8/4P3/3B4/PPP2PPP/RNBQK2R b KQkq -" => vec!["d7d5"],
        "rnbqkb1r/pppp1ppp/5n2/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq -" => vec!["f3e5", "d2d4"],
        "rnbqkb1r/pppp1ppp/5n2/4N3/4P3/8/PPPP1PPP/RNBQKB1R b KQkq -" => vec!["d7d6"],
        "rnbqkb1r/ppp2ppp/3p1n2/4N3/4P3/8/PPPP1PPP/RNBQKB1R w KQkq -" => vec!["e5f3"],
        "rnbqkb1r/ppp2ppp/3p1n2/8/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq -" => vec!["f6e4"],
        "rnbqkb1r/ppp2ppp/3p4/8/4n3/5N2/PPPP1PPP/RNBQKB1R w KQkq -" => vec!["b1c3", "d2d4"],
        "rnbqkb1r/ppp2ppp/3p4/8/4n3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq -" => vec!["e4c3"],
        "rnbqkb1r/ppp2ppp/3p4/8/8/2n2N2/PPPP1PPP/R1BQKB1R w KQkq -" => vec!["d2c3"],
        "rnbqkb1r/ppp2ppp/3p4/8/8/2P2N2/PPP2PPP/R1BQKB1R b KQkq -" => vec!["f8e7"],
        "r1bqk2r/ppp1bppp/2n5/3p4/3Pn3/3B1N2/PPP2PPP/RNBQ1RK1 w kq -" => {
            vec!["b1d2", "c2c4", "b1c3"]
        }
        "r1bqk2r/ppp1bppp/2n5/3p4/3Pn3/2NB1N2/PPP2PPP/R1BQ1RK1 b kq -" => vec!["c8f5"],
        "r2qk2r/ppp1bppp/2n5/3p1b2/3Pn3/2NB1N2/PPP2PPP/R1BQ1RK1 w kq -" => vec!["f1e1"],
        "r2qk2r/ppp1bppp/2n5/3p1b2/3Pn3/2NB1N2/PPP2PPP/R1BQR1K1 b kq -" => vec!["e4c3"],
        "r2qk2r/ppp1bppp/2n5/3p1b2/3P4/2nB1N2/PPP2PPP/R1BQR1K1 w kq -" => vec!["b2c3"],
        "rnbqkb1r/pppp1ppp/5n2/4p3/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq -" => vec!["f6e4"],
        "rnbqkb1r/pppp1ppp/8/4p3/3Pn3/5N2/PPP2PPP/RNBQKB1R w KQkq -" => vec!["f1d3"],
        "rnbqkb1r/pppp1ppp/8/4p3/3Pn3/3B1N2/PPP2PPP/RNBQK2R b KQkq -" => vec!["d7d5"],
        "rnbqkb1r/ppp2ppp/8/3pp3/3Pn3/3B1N2/PPP2PPP/RNBQK2R w KQkq -" => vec!["f3e5"],
        "rnbqkb1r/ppp2ppp/8/3pN3/3Pn3/3B4/PPP2PPP/RNBQK2R b KQkq -" => vec!["b8d7"],
        "rnbqkbnr/pppp1ppp/8/4p3/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq -" => vec!["g8f6"],
        "rnbqkb1r/pppp1ppp/5n2/4p3/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq -" => vec!["f1c4"],
        "rnbqkb1r/pppp1ppp/5n2/4p3/2B1P3/2N5/PPPP1PPP/R1BQK1NR b KQkq -" => vec!["f8c5"],
        "rnbqk2r/pppp1ppp/5n2/2b1p3/2B1P3/2N5/PPPP1PPP/R1BQK1NR w KQkq -" => vec!["d2d3"],
        "rnbqk2r/pppp1ppp/5n2/2b1p3/2B1P3/2NP4/PPP2PPP/R1BQK1NR b KQkq -" => vec!["b8c6"],
        "r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B1P3/2NP4/PPP2PPP/R1BQK1NR w KQkq -" => vec!["g1f3"],
        "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq -" => vec!["g1f3", "c2c3", "b1c3"],
        "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq -" => {
            vec!["d7d6", "b8c6", "e7e6"]
        }
        "rnbqkbnr/pp2pppp/3p4/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq -" => vec!["d2d4"],
        "rnbqkbnr/pp2pppp/3p4/2p5/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq -" => vec!["c5d4"],
        "rnbqkbnr/pp2pppp/3p4/8/3pP3/5N2/PPP2PPP/RNBQKB1R w KQkq -" => vec!["f3d4"],
        "rnbqkbnr/pp2pppp/3p4/8/3NP3/8/PPP2PPP/RNBQKB1R b KQkq -" => vec!["g8f6"],
        "rnbqkb1r/pp2pppp/3p1n2/8/3NP3/8/PPP2PPP/RNBQKB1R w KQkq -" => vec!["b1c3"],
        "rnbqkb1r/pp2pppp/3p1n2/8/3NP3/2N5/PPP2PPP/R1BQKB1R b KQkq -" => vec!["a7a6"],
        "rnbqkb1r/1p2pppp/p2p1n2/8/3NP3/2N5/PPP2PPP/R1BQKB1R w KQkq -" => {
            vec!["c1e3", "c1g5", "h2h3", "f2f3"]
        }
        "rnbqkb1r/1p2pppp/p2p1n2/8/3NP3/2N1B3/PPP2PPP/R2QKB1R b KQkq -" => {
            vec!["e7e5", "e7e6", "f6g4"]
        }
        "rnbqkb1r/1p3ppp/p2p1n2/4p3/3NP3/2N1B3/PPP2PPP/R2QKB1R w KQkq -" => vec!["d4b3"],
        "rnbqkb1r/1p3ppp/p2p1n2/4p3/4P3/1NN1B3/PPP2PPP/R2QKB1R b KQkq -" => vec!["c8e6", "f8e7"],
        "rn1qkb1r/1p3ppp/p2pbn2/4p3/4P3/1NN1B3/PPP2PPP/R2QKB1R w KQkq -" => vec!["f2f3", "f2f4"],
        "r2qkb1r/1p1n1p2/p2p1np1/3Pp2p/8/1N2BP2/PPPQ2PP/R3KB1R w KQkq -" => {
            vec!["b3a5", "e1c1", "f1e2"]
        }
        "rnbqkb1r/1p3ppp/p2ppn2/8/3NP3/2N1B3/PPP2PPP/R2QKB1R w KQkq -" => vec!["f1e2"],
        "rnbqkb1r/1p2pppp/p2p1n2/6B1/3NP3/2N5/PPP2PPP/R2QKB1R b KQkq -" => vec!["e7e6", "b8d7"],
        "rnbqkb1r/1p3ppp/p2ppn2/6B1/3NP3/2N5/PPP2PPP/R2QKB1R w KQkq -" => vec!["f2f4", "d1f3"],
        "rnbqkb1r/1p3ppp/p2ppn2/6B1/3NPP2/2N5/PPP3PP/R2QKB1R b KQkq -" => vec!["d8b6", "b8d7"],
        "rnb1kb1r/1p3ppp/pq1ppn2/6B1/3NPP2/2N5/PPP3PP/R2QKB1R w KQkq -" => vec!["d4b3", "d1d2"],
        "rnbqkb1r/1p2pppp/p2p1n2/8/3NP3/2N4P/PPP2PP1/R1BQKB1R b KQkq -" => vec!["e7e5", "e7e6"],
        "r1bqkb1r/pp2pppp/2np1n2/8/3NP3/2N5/PPP2PPP/R1BQKB1R w KQkq -" => vec!["f2f3", "c1g5"],
        "r1bqkbnr/pp1ppppp/2n5/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq -" => {
            vec!["f1b5", "d2d4", "b1c3"]
        }
        "r1bqkbnr/pp1ppppp/2n5/1Bp5/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq -" => vec!["g7g6"],
        "r1bqkbnr/pp1ppp1p/2n3p1/1Bp5/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq -" => vec!["b5c6", "e1g1"],
        "r1bqkbnr/pp1ppp1p/2n3p1/1Bp5/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq -" => vec!["f8g7"],
        "r1bqk1nr/pp1pppbp/2n3p1/1Bp5/4P3/5N2/PPPP1PPP/RNBQ1RK1 w kq -" => vec!["f1e1"],
        "r1bqk1nr/pp1pppbp/2n3p1/1Bp5/4P3/5N2/PPPP1PPP/RNBQR1K1 b kq -" => vec!["g8f6"],
        "r1bqkbnr/pp1ppppp/2n5/2p5/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq -" => vec!["c5d4"],
        "r1bqkbnr/pp1ppppp/2n5/8/3pP3/5N2/PPP2PPP/RNBQKB1R w KQkq -" => vec!["f3d4"],
        "r1bqkbnr/pp1ppppp/2n5/8/3NP3/8/PPP2PPP/RNBQKB1R b KQkq -" => vec!["g8f6"],
        "r1bqkb1r/pp1ppppp/2n2n2/8/3NP3/8/PPP2PPP/RNBQKB1R w KQkq -" => vec!["b1c3"],
        "r1bqkb1r/pp1ppppp/2n2n2/8/3NP3/2N5/PPP2PPP/R1BQKB1R b KQkq -" => vec!["e7e5", "d7d6"],
        "r1bqkb1r/pp3ppp/2np1n2/1N2p3/4P3/2N5/PPP2PPP/R1BQKB1R w KQkq -" => vec!["c1g5", "c3d5"],
        "r1bqkb1r/5ppp/p1np1n2/1p2p1B1/4P3/N1N5/PPP2PPP/R2QKB1R w KQkq -" => vec!["g5f6", "c3d5"],
        "r1bqkb1r/pp1p1ppp/2n1pn2/8/3NP3/2N5/PPP2PPP/R1BQKB1R w KQkq -" => vec!["d4c6", "d4b5"],
        "r1bqkb1r/pp1p1ppp/2N1pn2/8/4P3/2N5/PPP2PPP/R1BQKB1R b KQkq -" => vec!["b7c6"],
        "r1bqkb1r/p2p1ppp/2p1pn2/8/4P3/2N5/PPP2PPP/R1BQKB1R w KQkq -" => vec!["e4e5"],
        "r1bqkb1r/p2p1ppp/2p1pn2/4P3/8/2N5/PPP2PPP/R1BQKB1R b KQkq -" => vec!["f6d5"],
        "r1bqkb1r/p2p1ppp/2p1p3/3nP3/8/2N5/PPP2PPP/R1BQKB1R w KQkq -" => vec!["c3e4"],
        "r1bqkb1r/pp1p1ppp/2n1pn2/1N6/4P3/2N5/PPP2PPP/R1BQKB1R b KQkq -" => vec!["d7d6"],
        "r1bqkb1r/pp3ppp/2nppn2/1N6/4P3/2N5/PPP2PPP/R1BQKB1R w KQkq -" => vec!["c1f4"],
        "r1bqkb1r/pp3ppp/2nppn2/1N6/4PB2/2N5/PPP2PPP/R2QKB1R b KQkq -" => vec!["e6e5"],
        "r1bqkb1r/pp3ppp/2np1n2/1N2p3/4PB2/2N5/PPP2PPP/R2QKB1R w KQkq -" => vec!["f4g5"],
        "r1bqkbnr/pp1ppppp/2n5/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq -" => vec!["e7e5", "e7e6"],
        "r1bq1rk1/pp2bppp/2np1n2/2p1p3/P1B1P3/2NP1N1P/1PP2PP1/R1BQ1RK1 b - -" => {
            vec!["c8e6", "h7h6"]
        }
        "r1b1kbnr/1pqp1ppp/p1n1p3/8/3NP3/2N1B3/PPP2PPP/R2QKB1R w KQkq -" => {
            vec!["f2f4", "d1d2", "f1e2"]
        }
        "r1b1kbnr/1pqp1ppp/p1n1p3/8/3NP3/2N1B3/PPP1BPPP/R2QK2R b KQkq -" => vec!["g8f6"],
        "r1b1kb1r/1pqp1ppp/p1n1pn2/8/3NP3/2N1B3/PPP1BPPP/R2QK2R w KQkq -" => vec!["e1g1"],
        "rnbqkbnr/pp1ppppp/8/2p5/4P3/2P5/PP1P1PPP/RNBQKBNR b KQkq -" => vec!["d7d5"],
        "rnbqkbnr/pp2pppp/8/2pp4/4P3/2P5/PP1P1PPP/RNBQKBNR w KQkq -" => vec!["e4d5"],
        "rnbqkbnr/pp2pppp/8/2pP4/8/2P5/PP1P1PPP/RNBQKBNR b KQkq -" => vec!["d8d5"],
        "rnb1kbnr/pp2pppp/8/2pq4/8/2P5/PP1P1PPP/RNBQKBNR w KQkq -" => vec!["d2d4"],
        "rnb1kbnr/pp2pppp/8/2pq4/3P4/2P5/PP3PPP/RNBQKBNR b KQkq -" => vec!["g8f6"],
        "rnb1kb1r/pp2pppp/5n2/2pq4/3P4/2P5/PP3PPP/RNBQKBNR w KQkq -" => vec!["g1f3"],
        "rnbqkbnr/pp1ppppp/8/2p5/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq -" => {
            vec!["e7e6", "g7g6", "d7d6", "b8c6"]
        }
        "rnbqkbnr/pp2pppp/3p4/2p5/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq -" => vec!["g1f3", "g1e2"],
        "rnbqkbnr/pp2pppp/3p4/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq -" => vec!["a7a6", "e7e5"],
        "rnbqkbnr/1p2pppp/p2p4/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq -" => vec!["d2d4"],
        "rnbqkbnr/1p2pppp/p2p4/2p5/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq -" => vec!["c5d4"],
        "rnbqkbnr/1p2pppp/p2p4/8/3pP3/2N2N2/PPP2PPP/R1BQKB1R w KQkq -" => vec!["f3d4"],
        "rnbqkbnr/1p2pppp/p2p4/8/3NP3/2N5/PPP2PPP/R1BQKB1R b KQkq -" => vec!["g8f6"],
        "rnbqkbnr/pp3ppp/3p4/2p1p3/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq -" => vec!["f1c4"],
        "rnbqkbnr/pp3ppp/3p4/2p1p3/2B1P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq -" => vec!["f8e7"],
        "rnbqk1nr/pp2bppp/3p4/2p1p3/2B1P3/2N2N2/PPPP1PPP/R1BQK2R w KQkq -" => vec!["e1g1"],
        "rnbqk1nr/pp2bppp/3p4/2p1p3/2B1P3/2N2N2/PPPP1PPP/R1BQ1RK1 b kq -" => vec!["g8f6"],
        "rnbqk2r/pp2bppp/3p1n2/2p1p3/2B1P3/2N2N2/PPPP1PPP/R1BQ1RK1 w kq -" => vec!["d2d3"],
        "rnbqk2r/pp2bppp/3p1n2/2p1p3/2B1P3/2NP1N2/PPP2PPP/R1BQ1RK1 b kq -" => vec!["b8c6"],
        "rnbqkbnr/pp2pppp/3p4/2p5/4P3/2N5/PPPPNPPP/R1BQKB1R b KQkq -" => {
            vec!["a7a6", "b8c6", "g8f6"]
        }
        "rnbqkbnr/1p2pppp/p2p4/2p5/4P3/2N5/PPPPNPPP/R1BQKB1R w KQkq -" => vec!["d2d4"],
        "rnbqkbnr/1p2pppp/p2p4/2p5/3PP3/2N5/PPP1NPPP/R1BQKB1R b KQkq -" => vec!["c5d4"],
        "rnbqkbnr/1p2pppp/p2p4/8/3pP3/2N5/PPP1NPPP/R1BQKB1R w KQkq -" => vec!["e2d4"],
        "r1bqkbnr/pp2pppp/2np4/2p5/4P3/2N5/PPPPNPPP/R1BQKB1R w KQkq -" => vec!["d2d4"],
        "r1bqkbnr/pp2pppp/2np4/2p5/3PP3/2N5/PPP1NPPP/R1BQKB1R b KQkq -" => vec!["c5d4"],
        "r1bqkbnr/pp2pppp/2np4/8/3pP3/2N5/PPP1NPPP/R1BQKB1R w KQkq -" => vec!["e2d4"],
        "r1bqkbnr/pp2pppp/2np4/8/3NP3/2N5/PPP2PPP/R1BQKB1R b KQkq -" => vec!["e7e5", "g8f6"],
        "r1bqkbnr/pp3ppp/2np4/4p3/3NP3/2N5/PPP2PPP/R1BQKB1R w KQkq -" => vec!["d4b5"],
        "r1bqkbnr/pp3ppp/2np4/1N2p3/3NP3/8/PPP2PPP/R1BQKB1R b KQkq -" => vec!["g8f6"],
        "r1bqkb1r/pp3ppp/2np1n2/1N2p3/3NP3/8/PPP2PPP/R1BQKB1R w KQkq -" => vec!["c1g5"],
        "r1bqkb1r/pp3ppp/2np1n2/1N2p1B1/3NP3/8/PPP2PPP/R2QKB1R b KQkq -" => vec!["a7a6"],
        "r1bqkb1r/1p3ppp/p1np1n2/1N2p1B1/3NP3/8/PPP2PPP/R2QKB1R w KQkq -" => vec!["b5a3"],
        "r1bqkbnr/pp3ppp/2np4/1N2p3/4P3/2N5/PPP2PPP/R1BQKB1R b KQkq -" => vec!["g8f6"],
        "rnbqkb1r/pp2pppp/3p1n2/2p5/4P3/2N5/PPPPNPPP/R1BQKB1R w KQkq -" => vec!["d2d4"],
        "rnbqkb1r/pp2pppp/3p1n2/2p5/3PP3/2N5/PPP1NPPP/R1BQKB1R b KQkq -" => vec!["c5d4"],
        "rnbqkb1r/pp2pppp/3p1n2/8/3pP3/2N5/PPP1NPPP/R1BQKB1R w KQkq -" => vec!["e2d4"],
        "r1bqkbnr/pp1ppppp/2n5/2p5/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq -" => vec!["g1f3", "g1e2"],
        "r1bqkbnr/pp1ppppp/2n5/2p5/4P3/2N5/PPPPNPPP/R1BQKB1R b KQkq -" => vec!["d7d6"],
        "rnbqkbnr/pp1ppppp/2p5/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq -" => vec!["d2d4", "b1c3"],
        "rnbqkbnr/pp1ppppp/2p5/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq -" => vec!["d7d5"],
        "rnbqkbnr/pp2pppp/2p5/3p4/3PP3/8/PPP2PPP/RNBQKBNR w KQkq -" => vec!["e4e5", "b1c3"],
        "rnbqkbnr/pp2pppp/2p5/3pP3/3P4/8/PPP2PPP/RNBQKBNR b KQkq -" => vec!["c8f5"],
        "rn1qkbnr/pp2pppp/2p5/3pPb2/3P4/8/PPP2PPP/RNBQKBNR w KQkq -" => {
            vec!["g1f3", "h2h4", "c2c3"]
        }
        "rn1qkbnr/pp2pppp/2p5/3pPb2/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq -" => vec!["e7e6"],
        "rn1qkbnr/pp3ppp/2p1p3/3pPb2/3P4/5N2/PPP2PPP/RNBQKB1R w KQkq -" => vec!["f1e2"],
        "rn1qkbnr/pp3ppp/2p1p3/3pPb2/3P4/5N2/PPP1BPPP/RNBQK2R b KQkq -" => vec!["b8d7", "c6c5"],
        "r2qkbnr/pp1n1ppp/2p1p3/3pPb2/3P4/5N2/PPP1BPPP/RNBQK2R w KQkq -" => vec!["e1g1", "c2c3"],
        "r2qkbnr/pp1n1ppp/2p1p3/3pPb2/3P4/5N2/PPP1BPPP/RNBQ1RK1 b kq -" => vec!["g8e7"],
        "r2qkb1r/pp1nnpp1/2p1p2p/3pPb2/3P4/5N2/PPPNBPPP/R1BQ1RK1 w kq -" => {
            vec!["d2b3", "c2c3", "f1e1"]
        }
        "r3kbnr/ppqn2pp/2p1pp2/3pPb2/3P4/2P2N2/PP2BPPP/RNBQ1RK1 w kq -" => vec!["c1f4"],
        "rnbqkbnr/pp2pppp/2p5/3p4/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq -" => vec!["d5e4"],
        "rnbqkbnr/pp2pppp/2p5/8/3Pp3/2N5/PPP2PPP/R1BQKBNR w KQkq -" => vec!["c3e4"],
        "rnbqkbnr/pp2pppp/2p5/8/3PN3/8/PPP2PPP/R1BQKBNR b KQkq -" => vec!["c8f5"],
        "rn1qkbnr/pp2pppp/2p5/5b2/3PN3/8/PPP2PPP/R1BQKBNR w KQkq -" => vec!["e4g3"],
        "rn1qkbnr/pp2pppp/2p5/5b2/3P4/6N1/PPP2PPP/R1BQKBNR b KQkq -" => vec!["f5g6"],
        "rnbqkbnr/pp1ppppp/2p5/8/2P1P3/8/PP1P1PPP/RNBQKBNR b KQkq -" => vec!["d7d5"],
        "rnbqkbnr/pp2pppp/2p5/3p4/2P1P3/8/PP1P1PPP/RNBQKBNR w KQkq -" => vec!["e4d5"],
        "rnbqkbnr/pp2pppp/2p5/3P4/2P5/8/PP1P1PPP/RNBQKBNR b KQkq -" => vec!["c6d5"],
        "rnbqkbnr/pp2pppp/8/3p4/2P5/8/PP1P1PPP/RNBQKBNR w KQkq -" => vec!["c4d5", "d2d4"],
        "rnbqkbnr/pp2pppp/8/3P4/8/8/PP1P1PPP/RNBQKBNR b KQkq -" => vec!["g8f6"],
        "rnbqkb1r/pp2pppp/5n2/3P4/8/8/PP1P1PPP/RNBQKBNR w KQkq -" => vec!["b1c3"],
        "rnbqkb1r/pp2pppp/5n2/3P4/8/2N5/PP1P1PPP/R1BQKBNR b KQkq -" => vec!["f6d5"],
        "rnbqkb1r/pp2pppp/8/3n4/8/2N5/PP1P1PPP/R1BQKBNR w KQkq -" => vec!["g1f3"],
        "rnbqkbnr/pp2pppp/8/3p4/2PP4/8/PP3PPP/RNBQKBNR b KQkq -" => vec!["g8f6"],
        "rnbqkb1r/pp2pppp/5n2/3p4/2PP4/8/PP3PPP/RNBQKBNR w KQkq -" => vec!["b1c3"],
        "rnbqkb1r/pp2pppp/5n2/3p4/2PP4/2N5/PP3PPP/R1BQKBNR b KQkq -" => vec!["b8c6"],
        "r1bqkb1r/pp2pppp/2n2n2/3p4/2PP4/2N5/PP3PPP/R1BQKBNR w KQkq -" => vec!["g1f3"],
        "rnbqkbnr/pp1ppppp/2p5/8/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq -" => vec!["d7d5"],
        "rnbqkbnr/pp2pppp/2p5/3p4/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq -" => vec!["g1f3", "d2d4"],
        "rnbqkbnr/pppp1ppp/4p3/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq -" => vec!["d2d4"],
        "rnbqkbnr/pppp1ppp/4p3/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq -" => vec!["d7d5"],
        "rnbqkbnr/ppp2ppp/4p3/3p4/3PP3/8/PPP2PPP/RNBQKBNR w KQkq -" => vec!["e4e5", "b1c3"],
        "rnbqkbnr/ppp2ppp/4p3/3pP3/3P4/8/PPP2PPP/RNBQKBNR b KQkq -" => vec!["c7c5"],
        "rnbqkbnr/pp3ppp/4p3/2ppP3/3P4/8/PPP2PPP/RNBQKBNR w KQkq -" => vec!["c2c3"],
        "rnbqkbnr/pp3ppp/4p3/2ppP3/3P4/2P5/PP3PPP/RNBQKBNR b KQkq -" => vec!["b8c6"],
        "r1bqkbnr/pp3ppp/2n1p3/2ppP3/3P4/2P5/PP3PPP/RNBQKBNR w KQkq -" => vec!["g1f3"],
        "r1bqkbnr/pp3ppp/2n1p3/2ppP3/3P4/2P2N2/PP3PPP/RNBQKB1R b KQkq -" => vec!["c8d7"],
        "r2qkbnr/pp1b1ppp/2n1p3/2ppP3/3P4/2P2N2/PP3PPP/RNBQKB1R w KQkq -" => vec!["f1e2"],
        "r2qkbnr/pp1b1ppp/2n1p3/2ppP3/3P4/2P2N2/PP2BPPP/RNBQK2R b KQkq -" => vec!["g8e7"],
        "r2qkb1r/pp1bnppp/2n1p3/2ppP3/3P4/2P2N2/PP2BPPP/RNBQK2R w KQkq -" => vec!["e1g1"],
        "r2qkb1r/pp1bnppp/2n1p3/2ppP3/3P4/2P2N2/PP2BPPP/RNBQ1RK1 b kq -" => vec!["e7g6"],
        "r2qkb1r/pp1b1ppp/2n1p1n1/2ppP3/3P4/2P2N2/PP2BPPP/RNBQ1RK1 w kq -" => {
            vec!["g2g3", "c1e3", "b1a3", "f1e1"]
        }
        "rnbqkbnr/ppp2ppp/4p3/3p4/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq -" => vec!["g8f6"],
        "rnbqkb1r/ppp2ppp/4pn2/3p4/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq -" => vec!["e4e5"],
        "rnbqkb1r/ppp2ppp/4pn2/3pP3/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq -" => vec!["f6d7"],
        "rnbqkb1r/pppn1ppp/4p3/3pP3/3P4/2N5/PPP2PPP/R1BQKBNR w KQkq -" => vec!["f2f4"],
        "rnbqkb1r/pppn1ppp/4p3/3pP3/3P1P2/2N5/PPP3PP/R1BQKBNR b KQkq -" => vec!["c7c5"],
        "rnbqkb1r/pp1n1ppp/4p3/2ppP3/3P1P2/2N5/PPP3PP/R1BQKBNR w KQkq -" => vec!["g1f3"],
        "rnbqkb1r/pp1n1ppp/4p3/2ppP3/3P1P2/2N2N2/PPP3PP/R1BQKB1R b KQkq -" => vec!["b8c6"],
        "r1bqkb1r/pp1n1ppp/2n1p3/2ppP3/3P1P2/2N2N2/PPP3PP/R1BQKB1R w KQkq -" => vec!["c1e3"],
        "r1bqkb1r/pp1n1ppp/2n1p3/2ppP3/3P1P2/2N1BN2/PPP3PP/R2QKB1R b KQkq -" => {
            vec!["f8e7", "a7a6"]
        }
        "rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R b KQkq -" => {
            vec!["d7d5", "e7e6", "c7c5", "g8f6"]
        }
        "rnbqkbnr/ppp1pppp/8/3p4/8/5N2/PPPPPPPP/RNBQKB1R w KQkq -" => vec!["d2d4", "g2g3", "c2c4"],
        "rnbqkbnr/ppp1pppp/8/3p4/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq -" => {
            vec!["g8f6", "e7e6", "c7c6"]
        }
        "rnbqkb1r/ppp1pppp/5n2/3p4/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq -" => vec!["c1f4", "c2c4"],
        "rnbqkb1r/ppp1pppp/5n2/3p4/3P1B2/5N2/PPP1PPPP/RN1QKB1R b KQkq -" => vec!["e7e6", "c7c5"],
        "rnbqkb1r/ppp2ppp/4pn2/3p4/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq -" => vec!["e2e3"],
        "rnbqkb1r/ppp2ppp/4pn2/3p4/3P1B2/4PN2/PPP2PPP/RN1QKB1R b KQkq -" => vec!["c7c5"],
        "rnbqkb1r/pp3ppp/4pn2/2pp4/3P1B2/4PN2/PPP2PPP/RN1QKB1R w KQkq -" => vec!["c2c3", "b1d2"],
        "rnbqkb1r/pp3ppp/4pn2/2pp4/3P1B2/2P1PN2/PP3PPP/RN1QKB1R b KQkq -" => vec!["f8d6"],
        "rnbqkb1r/pp3ppp/4pn2/2pp4/3P1B2/4PN2/PPPN1PPP/R2QKB1R b KQkq -" => vec!["b8c6"],
        "r1bqkb1r/pp3ppp/2n1pn2/2pp4/3P1B2/2P1PN2/PP1N1PPP/R2QKB1R b KQkq -" => {
            vec!["f8d6", "c5d4"]
        }
        "r1bq1rk1/pp3ppp/2nbpn2/2pp4/3P4/2PBPNB1/PP1N1PPP/R2QK2R b KQ -" => vec!["b7b6"],
        "rnbqkb1r/pp2pppp/5n2/2pp4/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq -" => vec!["e2e3"],
        "rnbqkb1r/pp2pppp/5n2/2pp4/3P1B2/4PN2/PPP2PPP/RN1QKB1R b KQkq -" => vec!["b8c6"],
        "r1bqkb1r/pp2pppp/2n2n2/2pp4/3P1B2/4PN2/PPP2PPP/RN1QKB1R w KQkq -" => vec!["b1d2"],
        "rnbqkb1r/ppp1pppp/5n2/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq -" => vec!["c7c6", "e7e6"],
        "rnbqkb1r/pp2pppp/2p2n2/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq -" => vec!["g2g3", "e2e3"],
        "rnbqkb1r/pp2pppp/2p2n2/3p4/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq -" => vec!["c8f5", "g7g6"],
        "rn1qkb1r/pp2pppp/2p2n2/3p1b2/2PP4/5NP1/PP2PP1P/RNBQKB1R w KQkq -" => vec!["f1g2"],
        "rn1qkb1r/pp2pppp/2p2n2/3p1b2/2PP4/5NP1/PP2PPBP/RNBQK2R b KQkq -" => vec!["e7e6"],
        "rnbqkb1r/pp2pp1p/2p2np1/3p4/2PP4/5NP1/PP2PP1P/RNBQKB1R w KQkq -" => vec!["f1g2"],
        "rnbqkb1r/pp2pp1p/2p2np1/3p4/2PP4/5NP1/PP2PPBP/RNBQK2R b KQkq -" => vec!["f8g7"],
        "rnbqk2r/pp2ppbp/2p2np1/3p4/2PP4/5NP1/PP2PPBP/RNBQK2R w KQkq -" => vec!["e1g1"],
        "rnbqk2r/pp2ppbp/2p2np1/3p4/2PP4/5NP1/PP2PPBP/RNBQ1RK1 b kq -" => vec!["e8g8"],
        "rnbqkb1r/pp2pppp/2p2n2/3p4/2PP4/4PN2/PP3PPP/RNBQKB1R b KQkq -" => {
            vec!["c8g4", "c8f5", "e7e6"]
        }
        "rn1qkb1r/pp2pppp/2p2n2/3p4/2PP2b1/4PN2/PP3PPP/RNBQKB1R w KQkq -" => vec!["h2h3"],
        "rn1qkb1r/pp2pppp/2p2n2/3p4/2PP2b1/4PN1P/PP3PP1/RNBQKB1R b KQkq -" => vec!["g4f3", "g4h5"],
        "rn1qkb1r/pp2pppp/2p2n2/3p1b2/2PP4/4PN2/PP3PPP/RNBQKB1R w KQkq -" => vec!["b1c3"],
        "rn1qkb1r/pp2pppp/2p2n2/3p1b2/2PP4/2N1PN2/PP3PPP/R1BQKB1R b KQkq -" => vec!["e7e6"],
        "rn1qkb1r/pp3ppp/2p1pn2/3p1b2/2PP4/2N1PN2/PP3PPP/R1BQKB1R w KQkq -" => vec!["f3h4"],
        "rnbqkb1r/pp3ppp/2p1pn2/3p4/2PP4/4PN2/PP3PPP/RNBQKB1R w KQkq -" => {
            vec!["d1c2", "f1d3", "b1d2"]
        }
        "r1bqkb1r/pp1n1ppp/2p1pn2/3p4/2PP4/1P2PN2/P1Q2PPP/RNB1KB1R b KQkq -" => {
            vec!["f8d6", "b7b6", "f6e4"]
        }
        "rnbqkb1r/pp3ppp/2p1pn2/3p4/2PP4/3BPN2/PP3PPP/RNBQK2R b KQkq -" => vec!["b8d7"],
        "r1bqkb1r/pp1n1ppp/2p1pn2/3p4/2PP4/3BPN2/PP3PPP/RNBQK2R w KQkq -" => vec!["e1g1"],
        "r1bqkb1r/pp1n1ppp/2p1pn2/3p4/2PP4/3BPN2/PP3PPP/RNBQ1RK1 b kq -" => vec!["d5c4"],
        "r1bqkb1r/pp1n1ppp/2p1pn2/8/2pP4/3BPN2/PP3PPP/RNBQ1RK1 w kq -" => vec!["d3c4"],
        "r1bqkb1r/pp1n1ppp/2p1pn2/8/2BP4/4PN2/PP3PPP/RNBQ1RK1 b kq -" => vec!["f8d6"],
        "rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq -" => {
            vec!["e2e3", "b1c3", "c1g5", "g2g3"]
        }
        "rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/4PN2/PP3PPP/RNBQKB1R b KQkq -" => vec!["c7c6", "b7b6"],
        "rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq -" => {
            vec!["f8e7", "c7c6", "f8b4", "d5c4"]
        }
        "rnbqk2r/ppp1bppp/4pn2/3p4/2PP4/2N2N2/PP2PPPP/R1BQKB1R w KQkq -" => {
            vec!["c1f4", "c1g5", "e2e3"]
        }
        "rnbqk2r/ppp1bppp/4pn2/3p4/2PP1B2/2N2N2/PP2PPPP/R2QKB1R b KQkq -" => vec!["e8g8"],
        "rnbq1rk1/ppp1bppp/4pn2/3p4/2PP1B2/2N2N2/PP2PPPP/R2QKB1R w KQ -" => vec!["e2e3"],
        "rnbq1rk1/ppp1bppp/4pn2/3p4/2PP1B2/2N1PN2/PP3PPP/R2QKB1R b KQ -" => vec!["b8d7"],
        "r1bq1rk1/pppnbppp/4pn2/3p4/2PP1B2/2N1PN2/PP3PPP/R2QKB1R w KQ -" => vec!["c4c5"],
        "r1bq1rk1/pppnbppp/4pn2/2Pp4/3P1B2/2N1PN2/PP3PPP/R2QKB1R b KQ -" => vec!["c7c6"],
        "rnbqk2r/ppp1bppp/4pn2/3p2B1/2PP4/2N2N2/PP2PPPP/R2QKB1R b KQkq -" => vec!["h7h6"],
        "rnbqk2r/ppp1bpp1/4pn1p/3p2B1/2PP4/2N2N2/PP2PPPP/R2QKB1R w KQkq -" => vec!["g5h4", "g5f6"],
        "rnbqk2r/ppp1bpp1/4pn1p/3p4/2PP3B/2N2N2/PP2PPPP/R2QKB1R b KQkq -" => vec!["e8g8"],
        "rnbq1rk1/ppp1bpp1/4pn1p/3p4/2PP3B/2N2N2/PP2PPPP/R2QKB1R w KQ -" => vec!["e2e3"],
        "rnbq1rk1/ppp1bpp1/4pn1p/3p4/2PP3B/2N1PN2/PP3PPP/R2QKB1R b KQ -" => vec!["b7b6"],
        "rnbqkb1r/pp3ppp/2p1pn2/3p4/2PP4/2N2N2/PP2PPPP/R1BQKB1R w KQkq -" => vec!["c1g5", "e2e3"],
        "rnbqkb1r/pp3ppp/2p1pn2/3p2B1/2PP4/2N2N2/PP2PPPP/R2QKB1R b KQkq -" => vec!["h7h6", "d5c4"],
        "rnbqkb1r/pp3pp1/2p1pn1p/3p2B1/2PP4/2N2N2/PP2PPPP/R2QKB1R w KQkq -" => vec!["g5f6", "g5h4"],
        "rnb1kb1r/pp3pp1/2p1pq1p/3p4/2PP4/2N2N2/PP2PPPP/R2QKB1R w KQkq -" => vec!["e2e3", "d1b3"],
        "rnb1kb1r/pp3pp1/2p1pq1p/3p4/2PP4/2N1PN2/PP3PPP/R2QKB1R b KQkq -" => vec!["g7g6", "b8d7"],
        "rnb1kb1r/pp3p2/2p1pqpp/3p4/2PP4/2N1PN2/PP3PPP/R2QKB1R w KQkq -" => vec!["f1e2", "f1d3"],
        "rnb1kb1r/pp3p2/2p1pqpp/3p4/2PP4/2N1PN2/PP2BPPP/R2QK2R b KQkq -" => vec!["f8g7", "b8d7"],
        "rnb1kb1r/pp3p2/2p1pqpp/3p4/2PP4/2NBPN2/PP3PPP/R2QK2R b KQkq -" => vec!["d5c4", "f8g7"],
        "r1b1kb1r/pp1n1pp1/2p1pq1p/3p4/2PP4/2N1PN2/PP3PPP/R2QKB1R w KQkq -" => vec!["f1e2", "f1d3"],
        "r1b1kb1r/pp1n1pp1/2p1pq1p/3p4/2PP4/2NBPN2/PP3PPP/R2QK2R b KQkq -" => vec!["d5c4"],
        "r1b1kb1r/pp1n1pp1/2p1pq1p/8/2pP4/2NBPN2/PP3PPP/R2QK2R w KQkq -" => vec!["d3c4"],
        "r1b1kb1r/pp1n1pp1/2p1pq1p/8/2BP4/2N1PN2/PP3PPP/R2QK2R b KQkq -" => vec!["g7g6"],
        "rnb1kb1r/pp3pp1/2p1pq1p/3p4/2PP4/1QN2N2/PP2PPPP/R3KB1R b KQkq -" => vec!["a7a5", "d5c4"],
        "rnbqkb1r/pp3ppp/2p1pn2/3p4/2PP4/2N1PN2/PP3PPP/R1BQKB1R b KQkq -" => vec!["b8d7"],
        "r1bqkb1r/pp1n1ppp/2p1pn2/3p4/2PP4/2N1PN2/PP3PPP/R1BQKB1R w KQkq -" => vec!["d1c2", "f1d3"],
        "r1bqk2r/pp1n1ppp/2pbpn2/3p4/2PP4/2N1PN2/PPQ2PPP/R1B1KB1R w KQkq -" => {
            vec!["f1d3", "b2b3", "f1e2"]
        }
        "r1bqk2r/pp1n1ppp/2pbpn2/3p4/2PP4/2N1PN2/PPQ1BPPP/R1B1K2R b KQkq -" => vec!["e8g8"],
        "r1bq1rk1/pp1n1ppp/2pbpn2/3p4/2PP4/2N1PN2/PPQ1BPPP/R1B1K2R w KQ -" => vec!["e1g1", "b2b3"],
        "r1bqkb1r/p2n1ppp/2p1pn2/1p6/3P4/2NBPN2/PP3PPP/R1BQK2R b KQkq -" => vec!["c8b7", "a7a6"],
        "rnbqkb1r/ppp2ppp/4pn2/3p2B1/2PP4/5N2/PP2PPPP/RN1QKB1R b KQkq -" => {
            vec!["f8e7", "h7h6", "d5c4"]
        }
        "rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq -" => vec!["f8b4"],
        "rnbqk2r/ppp2ppp/4pn2/3p4/1bPP4/5NP1/PP2PP1P/RNBQKB1R w KQkq -" => vec!["c1d2"],
        "rnbqk2r/ppp2ppp/4pn2/3p4/1bPP4/5NP1/PP1BPP1P/RN1QKB1R b KQkq -" => vec!["b4e7"],
        "rnbqk2r/ppp1bppp/4pn2/3p4/2PP4/5NP1/PP1BPP1P/RN1QKB1R w KQkq -" => vec!["f1g2"],
        "rnbqkbnr/ppp2ppp/4p3/3p4/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq -" => {
            vec!["e2e3", "c2c4", "g2g3"]
        }
        "rnbqkbnr/ppp2ppp/4p3/3p4/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq -" => vec!["g8f6"],
        "rnbqkb1r/ppp2ppp/4pn2/3p4/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq -" => vec!["b1d2", "c2c4"],
        "rnbqkb1r/ppp2ppp/4pn2/3p4/3P4/4PN2/PPPN1PPP/R1BQKB1R b KQkq -" => vec!["c7c5"],
        "rnbqkb1r/pp3ppp/4pn2/2pp4/3P4/4PN2/PPPN1PPP/R1BQKB1R w KQkq -" => vec!["b2b3"],
        "rnbqkb1r/pp3ppp/4pn2/2pp4/3P4/1P2PN2/P1PN1PPP/R1BQKB1R b KQkq -" => vec!["b7b6"],
        "rnbqkb1r/p4ppp/1p2pn2/2pp4/3P4/1P2PN2/P1PN1PPP/R1BQKB1R w KQkq -" => vec!["f1d3"],
        "rnbqkb1r/p4ppp/1p2pn2/2pp4/3P4/1P1BPN2/P1PN1PPP/R1BQK2R b KQkq -" => vec!["c8b7"],
        "rn1qkb1r/pb3ppp/1p2pn2/2pp4/3P4/1P1BPN2/P1PN1PPP/R1BQK2R w KQkq -" => vec!["c1b2"],
        "rnbqkbnr/ppp2ppp/4p3/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq -" => vec!["g8f6"],
        "rnbqkbnr/ppp2ppp/4p3/3p4/3P1B2/5N2/PPP1PPPP/RN1QKB1R b KQkq -" => vec!["g8f6"],
        "rnbqkbnr/ppp2ppp/4p3/3p4/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq -" => vec!["g8f6", "c7c5"],
        "rnbqkb1r/ppp2ppp/4pn2/3p4/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq -" => vec!["c2c4", "f1g2"],
        "rnbqkb1r/ppp2ppp/4pn2/3p4/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq -" => vec!["f8b4"],
        "rnbqk2r/ppp2ppp/4pn2/3p4/1b1P4/5NP1/PPP1PPBP/RNBQK2R w KQkq -" => vec!["c2c3"],
        "rnbqk2r/ppp2ppp/4pn2/3p4/1b1P4/2P2NP1/PP2PPBP/RNBQK2R b KQkq -" => vec!["b4e7"],
        "rnbqk2r/ppp1bppp/4pn2/3p4/3P4/2P2NP1/PP2PPBP/RNBQK2R w KQkq -" => vec!["c3c4"],
        "rnbqk2r/ppp1bppp/4pn2/3p4/2PP4/5NP1/PP2PPBP/RNBQK2R b KQkq -" => vec!["e8g8"],
        "rnbq1rk1/ppp1bppp/4pn2/3p4/2PP4/5NP1/PP2PPBP/RNBQK2R w KQ -" => vec!["e1g1"],
        "rnbq1rk1/ppp1bppp/4pn2/3p4/2PP4/5NP1/PP2PPBP/RNBQ1RK1 b - -" => vec!["d5c4"],
        "rnbq1rk1/ppp1bppp/4pn2/8/2pP4/5NP1/PP2PPBP/RNBQ1RK1 w - -" => vec!["d1c2"],
        "rnbq1rk1/ppp1bppp/4pn2/8/2pP4/5NP1/PPQ1PPBP/RNB2RK1 b - -" => vec!["a7a6"],
        "rnbq1rk1/1pp1bppp/p3pn2/8/2pP4/5NP1/PPQ1PPBP/RNB2RK1 w - -" => vec!["a2a4", "c2c4"],
        "rnbq1rk1/1pp1bppp/p3pn2/8/P1pP4/5NP1/1PQ1PPBP/RNB2RK1 b - -" => vec!["c8d7"],
        "rnbq1rk1/1pp1bppp/p3pn2/8/2QP4/5NP1/PP2PPBP/RNB2RK1 b - -" => vec!["b7b5"],
        "rnbq1rk1/2p1bppp/p3pn2/1p6/2QP4/5NP1/PP2PPBP/RNB2RK1 w - -" => vec!["c4c2"],
        "rnbq1rk1/2p1bppp/p3pn2/1p6/3P4/5NP1/PPQ1PPBP/RNB2RK1 b - -" => vec!["c8b7"],
        "rnbqkbnr/pp3ppp/4p3/2pp4/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq -" => vec!["f1g2"],
        "rnbqkbnr/pp3ppp/4p3/2pp4/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq -" => vec!["c5d4", "b8c6"],
        "rnbqkbnr/pp3ppp/4p3/3p4/3p4/5NP1/PPP1PPBP/RNBQK2R w KQkq -" => vec!["f3d4"],
        "rnbqkbnr/pp3ppp/4p3/3p4/3N4/6P1/PPP1PPBP/RNBQK2R b KQkq -" => vec!["g8f6"],
        "rnbqkb1r/pp3ppp/4pn2/3p4/3N4/6P1/PPP1PPBP/RNBQK2R w KQkq -" => vec!["e1g1"],
        "r1bqkbnr/pp3ppp/2n1p3/2pp4/3P4/5NP1/PPP1PPBP/RNBQK2R w KQkq -" => vec!["e1g1"],
        "r1bqkbnr/pp3ppp/2n1p3/2pp4/3P4/5NP1/PPP1PPBP/RNBQ1RK1 b kq -" => vec!["c5d4"],
        "r1bqkbnr/pp3ppp/2n1p3/3p4/3p4/5NP1/PPP1PPBP/RNBQ1RK1 w kq -" => vec!["f3d4"],
        "r1bqkbnr/pp3ppp/2n1p3/3p4/3N4/6P1/PPP1PPBP/RNBQ1RK1 b kq -" => vec!["f8c5"],
        "r1bqk1nr/pp3ppp/2n1p3/2bp4/3N4/6P1/PPP1PPBP/RNBQ1RK1 w kq -" => vec!["d4b3"],
        "r1bqk1nr/pp3ppp/2n1p3/2bp4/8/1N4P1/PPP1PPBP/RNBQ1RK1 b kq -" => vec!["c5b6"],
        "r1bqk1nr/pp3ppp/1bn1p3/3p4/8/1N4P1/PPP1PPBP/RNBQ1RK1 w kq -" => vec!["c2c4"],
        "r1bqk1nr/pp3ppp/1bn1p3/3p4/2P5/1N4P1/PP2PPBP/RNBQ1RK1 b kq -" => vec!["g8e7"],
        "rnbqkbnr/pp2pppp/2p5/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq -" => vec!["g8f6"],
        "rnbqkbnr/ppp1pppp/8/3p4/8/5NP1/PPPPPP1P/RNBQKB1R b KQkq -" => vec!["c7c6"],
        "rnbqkbnr/pp2pppp/2p5/3p4/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq -" => vec!["f1g2"],
        "rnbqkbnr/pp2pppp/2p5/3p4/8/5NP1/PPPPPPBP/RNBQK2R b KQkq -" => vec!["g8f6"],
        "rnbqkb1r/pp2pppp/2p2n2/3p4/8/5NP1/PPPPPPBP/RNBQK2R w KQkq -" => vec!["d2d3"],
        "rnbqkb1r/pp2pppp/2p2n2/3p4/8/3P1NP1/PPP1PPBP/RNBQK2R b KQkq -" => vec!["c8g4", "c8f5"],
        "rn1qkb1r/pp2pppp/2p2n2/3p4/6b1/3P1NP1/PPP1PPBP/RNBQK2R w KQkq -" => vec!["e1g1"],
        "rn1qkb1r/pp2pppp/2p2n2/3p4/6b1/3P1NP1/PPP1PPBP/RNBQ1RK1 b kq -" => vec!["b8d7"],
        "r2qkb1r/pp1npppp/2p2n2/3p4/6b1/3P1NP1/PPP1PPBP/RNBQ1RK1 w kq -" => vec!["c2c4"],
        "r2qkb1r/pp1npppp/2p2n2/3p4/2P3b1/3P1NP1/PP2PPBP/RNBQ1RK1 b kq -" => vec!["e7e6"],
        "rn1qkb1r/pp2pppp/2p2n2/3p1b2/8/3P1NP1/PPP1PPBP/RNBQK2R w KQkq -" => vec!["e1g1"],
        "rn1qkb1r/pp2pppp/2p2n2/3p1b2/8/3P1NP1/PPP1PPBP/RNBQ1RK1 b kq -" => vec!["b8d7"],
        "r2qkb1r/pp1npppp/2p2n2/3p1b2/8/3P1NP1/PPP1PPBP/RNBQ1RK1 w kq -" => vec!["b1d2"],
        "r2qkb1r/pp1npppp/2p2n2/3p1b2/8/3P1NP1/PPPNPPBP/R1BQ1RK1 b kq -" => vec!["e7e6"],
        "rnbqkbnr/ppp1pppp/8/3p4/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq -" => vec!["d5d4"],
        "rnbqkbnr/ppp1pppp/8/8/2Pp4/5N2/PP1PPPPP/RNBQKB1R w KQkq -" => vec!["b2b4"],
        "rnbqkbnr/pppp1ppp/4p3/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq -" => vec!["d2d4", "c2c4"],
        "rnbqkbnr/pppp1ppp/4p3/8/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq -" => vec!["d7d5"],
        "rnbqkbnr/pppp1ppp/4p3/8/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq -" => vec!["d7d5", "g8f6"],
        "rnbqkbnr/ppp2ppp/4p3/3p4/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq -" => vec!["d2d4"],
        "rnbqkb1r/pppp1ppp/4pn2/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq -" => {
            vec!["d7d5", "b7b6", "c7c6"]
        }
        "rnbqkb1r/p1pp1ppp/1p2pn2/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq -" => vec!["b1c3", "g2g3"],
        "rnbqkb1r/p1pp1ppp/1p2pn2/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq -" => vec!["c8b7"],
        "rn1qkb1r/pbpp1ppp/1p2pn2/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R w KQkq -" => vec!["a2a3"],
        "rn1qkb1r/pbpp1ppp/1p2pn2/8/2PP4/P1N2N2/1P2PPPP/R1BQKB1R b KQkq -" => vec!["d7d5"],
        "rn1qkb1r/pbp2ppp/1p2pn2/3p4/2PP4/P1N2N2/1P2PPPP/R1BQKB1R w KQkq -" => vec!["c4d5"],
        "rn1qkb1r/pbp2ppp/1p2pn2/3P4/3P4/P1N2N2/1P2PPPP/R1BQKB1R b KQkq -" => vec!["f6d5"],
        "rn1qkb1r/pbp2ppp/1p2p3/3n4/3P4/P1N2N2/1P2PPPP/R1BQKB1R w KQkq -" => vec!["e2e3"],
        "rnbqkb1r/p1pp1ppp/1p2pn2/8/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq -" => vec!["c8a6"],
        "rn1qkb1r/p1pp1ppp/bp2pn2/8/2PP4/5NP1/PP2PP1P/RNBQKB1R w KQkq -" => vec!["b2b3", "d1c2"],
        "rn1qkb1r/p1pp1ppp/bp2pn2/8/2PP4/1P3NP1/P3PP1P/RNBQKB1R b KQkq -" => vec!["f8b4"],
        "rn1qk2r/p1pp1ppp/bp2pn2/8/1bPP4/1P3NP1/P3PP1P/RNBQKB1R w KQkq -" => vec!["c1d2"],
        "rn1qk2r/p1pp1ppp/bp2pn2/8/1bPP4/1P3NP1/P2BPP1P/RN1QKB1R b KQkq -" => vec!["b4e7"],
        "rn1qk2r/p1ppbppp/bp2pn2/8/2PP4/1P3NP1/P2BPP1P/RN1QKB1R w KQkq -" => vec!["f1g2"],
        "rn1qk2r/p1ppbppp/bp2pn2/8/2PP4/1P3NP1/P2BPPBP/RN1QK2R b KQkq -" => {
            vec!["c7c6", "d7d5", "e8g8"]
        }
        "rn1qk2r/p3bppp/bpp1pn2/3p4/2PP4/1PB2NP1/P3PPBP/RN1QK2R w KQkq -" => {
            vec!["f3e5", "b1d2", "e1g1"]
        }
        "rn1qkb1r/p1pp1ppp/bp2pn2/8/2PP4/5NP1/PPQ1PP1P/RNB1KB1R b KQkq -" => vec!["c7c5"],
        "rn1qkb1r/p2p1ppp/bp2pn2/2p5/2PP4/5NP1/PPQ1PP1P/RNB1KB1R w KQkq -" => vec!["d4d5"],
        "rn1qkb1r/p2p1ppp/bp2pn2/2pP4/2P5/5NP1/PPQ1PP1P/RNB1KB1R b KQkq -" => vec!["e6d5"],
        "rn1qkb1r/p2p1ppp/bp3n2/2pp4/2P5/5NP1/PPQ1PP1P/RNB1KB1R w KQkq -" => vec!["c4d5"],
        "rn1qkb1r/p2p1ppp/bp3n2/2pP4/8/5NP1/PPQ1PP1P/RNB1KB1R b KQkq -" => vec!["a6b7"],
        "rn1qkb1r/pb1p1ppp/1p3n2/2pP4/8/5NP1/PPQ1PP1P/RNB1KB1R w KQkq -" => vec!["f1g2"],
        "rn1qkb1r/pb1p1ppp/1p3n2/2pP4/8/5NP1/PPQ1PPBP/RNB1K2R b KQkq -" => vec!["f6d5"],
        "rn1qkb1r/pb1p1ppp/1p6/2pn4/8/5NP1/PPQ1PPBP/RNB1K2R w KQkq -" => vec!["e1g1"],
        "rnbqkb1r/pp1p1ppp/2p1pn2/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq -" => vec!["c1f4"],
        "rnbqkb1r/pp1p1ppp/2p1pn2/8/2PP1B2/5N2/PP2PPPP/RN1QKB1R b KQkq -" => vec!["d7d5"],
        "rnbqkb1r/pp3ppp/2p1pn2/3p4/2PP1B2/5N2/PP2PPPP/RN1QKB1R w KQkq -" => vec!["e2e3"],
        "rnbqkb1r/pp3ppp/2p1pn2/3p4/2PP1B2/4PN2/PP3PPP/RN1QKB1R b KQkq -" => vec!["f8d6"],
        "rnbqk2r/pp3ppp/2pbpn2/3p4/2PP1B2/4PN2/PP3PPP/RN1QKB1R w KQkq -" => vec!["f4d6"],
        "rnbqk2r/pp3ppp/2pBpn2/3p4/2PP4/4PN2/PP3PPP/RN1QKB1R b KQkq -" => vec!["d8d6"],
        "rnb1k2r/pp3ppp/2pqpn2/3p4/2PP4/4PN2/PP3PPP/RN1QKB1R w KQkq -" => vec!["b1c3"],
        "rnbqkbnr/pp1ppppp/8/2p5/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq -" => vec!["b8c6"],
        "r1bqkbnr/pp1ppppp/2n5/2p5/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq -" => vec!["g2g3"],
        "rnbqkb1r/pppppppp/5n2/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq -" => vec!["d2d4"],
        "rnbqkb1r/pppppppp/5n2/8/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq -" => {
            vec!["d7d5", "e7e6", "c7c6"]
        }
        "rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq -" => vec!["g8f6", "d7d5"],
        "rnbqkb1r/pppppppp/5n2/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq -" => vec!["c2c4", "g1f3"],
        "rnbqkb1r/pppppppp/5n2/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq -" => vec!["e7e6", "g7g6"],
        "rnbqkb1r/pppp1ppp/4pn2/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq -" => vec!["g1f3", "b1c3"],
        "rnbqkb1r/pppp1ppp/4pn2/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq -" => vec!["f8b4"],
        "rnbqk2r/pppp1ppp/4pn2/8/1bPP4/2N5/PP2PPPP/R1BQKBNR w KQkq -" => vec!["g1f3", "d1c2"],
        "rnbqk2r/pppp1ppp/4pn2/8/1bPP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq -" => vec!["b7b6", "d7d5"],
        "rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq -" => {
            vec!["g1f3", "c4d5", "c1g5"]
        }
        "rnbqkb1r/pppppp1p/5np1/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq -" => vec!["f8g7", "d7d5"],
        "rnbqk2r/ppppppbp/5np1/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq -" => vec!["g1f3"],
        "rnbqk2r/ppppppbp/5np1/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq -" => vec!["e8g8", "d7d5"],
        "rnbq1rk1/ppppppbp/5np1/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R w KQ -" => vec!["e2e4"],
        "rnbq1rk1/ppppppbp/5np1/8/2PPP3/2N2N2/PP3PPP/R1BQKB1R b KQ -" => vec!["d7d6"],
        "rnbq1rk1/ppp1ppbp/3p1np1/8/2PPP3/2N2N2/PP3PPP/R1BQKB1R w KQ -" => vec!["f1e2", "h2h3"],
        "rnbq1rk1/ppp1ppbp/3p1np1/8/2PPP3/2N2N2/PP2BPPP/R1BQK2R b KQ -" => vec!["e7e5"],
        "rnbq1rk1/ppp2pbp/3p1np1/4p3/2PPP3/2N2N2/PP2BPPP/R1BQK2R w KQ -" => vec!["d4d5", "e1g1"],
        "rnbq1rk1/ppp2pbp/3p1np1/3Pp3/2P1P3/2N2N2/PP2BPPP/R1BQK2R b KQ -" => vec!["a7a5", "f6h5"],
        "rnbq1rk1/ppp1ppbp/3p1np1/8/2PPP3/2N2N1P/PP3PP1/R1BQKB1R b KQ -" => vec!["e7e5"],
        "rnbq1rk1/ppp2pbp/3p1np1/4p3/2PPP3/2N2N1P/PP3PP1/R1BQKB1R w KQ -" => vec!["d4d5"],
        "rnbq1rk1/ppp2pbp/3p1np1/3Pp3/2P1P3/2N2N1P/PP3PP1/R1BQKB1R b KQ -" => vec!["f6h5"],
        "rnbqk2r/ppp1ppbp/5np1/3p4/2PP4/2N2N2/PP2PPPP/R1BQKB1R w KQkq -" => vec!["c4d5", "d1b3"],
        "rnbqkbnr/ppp1pppp/8/3p4/3P4/8/PPP1PPPP/RNBQKBNR w KQkq -" => vec!["c2c4", "g1f3"],
        "rnbqkbnr/ppp1pppp/8/3p4/2PP4/8/PP2PPPP/RNBQKBNR b KQkq -" => vec!["e7e6", "c7c6"],
        "rnbqkbnr/ppp2ppp/4p3/3p4/2PP4/8/PP2PPPP/RNBQKBNR w KQkq -" => vec!["b1c3", "c4d5", "g1f3"],
        "rnbqkbnr/ppp2ppp/4p3/3p4/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq -" => vec!["g8f6"],
        "rnbqk1nr/pp3ppp/2p1p3/8/1bPPN3/8/PP3PPP/R1BQKBNR w KQkq -" => vec!["c1d2", "e4c3"],
        "rnbqkbnr/pp2pppp/2p5/3p4/2PP4/8/PP2PPPP/RNBQKBNR w KQkq -" => {
            vec!["g1f3", "b1c3", "e2e3", "c4d5"]
        }
        "rnbqkbnr/pp2pppp/2p5/3p4/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq -" => vec!["e7e6", "g8f6"],
        "rnbqkbnr/pp2pppp/2p5/3p4/2PP4/4P3/PP3PPP/RNBQKBNR b KQkq -" => vec!["g8f6"],
        "rnbqkbnr/pp2pppp/2p5/3P4/3P4/8/PP2PPPP/RNBQKBNR b KQkq -" => vec!["c6d5"],
        "rnbqkbnr/pp2pppp/8/3p4/3P4/8/PP2PPPP/RNBQKBNR w KQkq -" => vec!["c1f4"],
        "rnbqkbnr/pp2pppp/8/3p4/3P1B2/8/PP2PPPP/RN1QKBNR b KQkq -" => vec!["b8c6"],
        "r1bqkbnr/pp2pppp/2n5/3p4/3P1B2/8/PP2PPPP/RN1QKBNR w KQkq -" => vec!["e2e3"],
        "r1bqkbnr/pp2pppp/2n5/3p4/3P1B2/4P3/PP3PPP/RN1QKBNR b KQkq -" => vec!["g8f6"],
        "r1bqkb1r/pp2pppp/2n2n2/3p4/3P1B2/4P3/PP3PPP/RN1QKBNR w KQkq -" => vec!["b1c3"],
        "r1bqkb1r/pp2pppp/2n2n2/3p4/3P1B2/2N1P3/PP3PPP/R2QKBNR b KQkq -" => vec!["a7a6", "c8f5"],
        "r1bqkb1r/1p2pppp/p1n2n2/3p4/3P1B2/2N1P3/PP3PPP/R2QKBNR w KQkq -" => vec!["g1f3"],
        "r1bqkb1r/1p2pppp/p1n2n2/3p4/3P1B2/2N1PN2/PP3PPP/R2QKB1R b KQkq -" => vec!["c8g4"],
        "r2qkb1r/pp2pppp/2n2n2/3p1b2/3P1B2/2N1P3/PP3PPP/R2QKBNR w KQkq -" => vec!["g1f3"],
        "r2qkb1r/pp2pppp/2n2n2/3p1b2/3P1B2/2N1PN2/PP3PPP/R2QKB1R b KQkq -" => vec!["e7e6"],
        "rnbqkbnr/pppppppp/8/8/2P5/8/PP1PPPPP/RNBQKBNR b KQkq -" => {
            vec!["g8f6", "e7e5", "c7c5", "e7e6", "c7c6"]
        }
        "rnbqkb1r/pppppppp/5n2/8/2P5/8/PP1PPPPP/RNBQKBNR w KQkq -" => vec!["b1c3", "d2d4"],
        "rnbqkb1r/pppppppp/5n2/8/2P5/2N5/PP1PPPPP/R1BQKBNR b KQkq -" => vec!["g7g6"],
        "rnbqkb1r/pppppp1p/5np1/8/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq -" => vec!["d2d4"],
        "rnbqkbnr/pppp1ppp/8/4p3/2P5/8/PP1PPPPP/RNBQKBNR w KQkq -" => vec!["b1c3"],
        "rnbqkbnr/pppp1ppp/8/4p3/2P5/2N5/PP1PPPPP/R1BQKBNR b KQkq -" => vec!["g8f6"],
        "rnbqkb1r/pppp1ppp/5n2/4p3/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq -" => vec!["g1f3"],
        "rnbqkb1r/pppp1ppp/5n2/4p3/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq -" => vec!["b8c6"],
        "r1bqkb1r/pppp1ppp/2n2n2/4p3/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq -" => vec!["g2g3", "e2e4"],
        "r1bqkb1r/pppp1ppp/2n2n2/4p3/2P5/2N2NP1/PP1PPP1P/R1BQKB1R b KQkq -" => vec!["f8b4", "d7d5"],
        "r1bqk2r/ppp1bppp/1nn5/4p3/8/2NP1NP1/PP2PPBP/R1BQ1RK1 b kq -" => vec!["e8g8", "c8e6"],
        "r1bqk2r/ppp2ppp/1bnp1n2/4p3/1PP1P3/P1NPBN2/5PPP/R2QKB1R b KQkq -" => vec!["b6e3", "c8g4"],
        "rnbqkbnr/pp1ppppp/8/2p5/2P5/8/PP1PPPPP/RNBQKBNR w KQkq -" => vec!["g1f3"],
        "rnbqkbnr/pppp1ppp/4p3/8/2P5/8/PP1PPPPP/RNBQKBNR w KQkq -" => vec!["d2d4"],
        "rnbqkbnr/pp1ppppp/2p5/8/2P5/8/PP1PPPPP/RNBQKBNR w KQkq -" => {
            vec!["e2e4", "d2d4", "g1f3", "b1c3"]
        }
        "rnbqkbnr/pp1ppppp/2p5/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq -" => vec!["d7d5"],
        "rnbqkbnr/pp1ppppp/2p5/8/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq -" => vec!["g8f6"],
        "rnbqkb1r/pp1ppppp/2p2n2/8/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq -" => vec!["d2d4", "b1c3"],
        "rnbqkb1r/pp2pppp/2p2n2/3p4/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq -" => vec!["e2e3", "d2d4"],
        "rnbqkb1r/pp1ppppp/2p2n2/8/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq -" => vec!["g1f3", "d2d4"],
        _ => Vec::default(),
    }
}
