let aoc identifier input =
  match identifier with
  | "01" -> Aoc2023.Day01.main input
  | "02part1" -> Aoc2023.Day02part1.main input
  | "02part2" -> Aoc2023.Day02part2.main input
  | "03part1" -> Aoc2023.Day03part1.main input
  | "03part2" -> Aoc2023.Day03part2.main input
  | "04part1" -> Aoc2023.Day04part1.main input
  | "04part2" -> Aoc2023.Day04part2.main input
  | "05part1" -> Aoc2023.Day05part1.main input
  | "05part2" -> Aoc2023.Day05part2.main input
  | "06part1" -> Aoc2023.Day06part1.main input
  | "06part2" -> Aoc2023.Day06part2.main input
  | "07part1" -> Aoc2023.Day07part1.main input
  | "07part2" -> Aoc2023.Day07part2.main input
  | "08part1" -> Aoc2023.Day08part1.main input
  | "08part2" -> Aoc2023.Day08part2.main input
  | "09part1" -> Aoc2023.Day09part1.main input
  | "09part2" -> Aoc2023.Day09part2.main input
  | "10" -> Aoc2023.Day10.main input
  | "11" -> Aoc2023.Day11.main input
  | _ -> print_endline "Not implemented yet"


open Cmdliner

let identifier =
  let doc = "Identifier" in
  Arg.(required & pos 0 (some string) None & info [] ~doc ~docv:"IDENTIFIER")

let input =
  let doc = "Input file" in
  Arg.(required & pos 1 (some file) None & info [] ~doc ~docv:"INPUT")

let cmd =
  let doc = "Solve Advent of Code puzzles" in
  let man = [
    `S Manpage.s_description;
    `P "$(tname) solves the Advent of Code puzzles.";
    `P "To solve the puzzle for day $(i,identifier), use the command:";
    `Pre "$(mname) $(i,identifier) $(i,input)";
    `P "where $(i,input) is the input file for the puzzle.";
    `P "The input file is expected to be in the current directory.";
    `S Manpage.s_bugs; `P "Report bugs to <tomas.sandrini@seznam.cz>.";
    `S Manpage.s_see_also; `P "https://github.com/tsandrini/aoc2023/" ]
  in
  let info = Cmd.info "aoc" ~version:"%%VERSION%%" ~doc ~man in
  Cmd.v info Term.(const aoc $ identifier $ input)

let main () = exit (Cmd.eval cmd)
let () = main ()
