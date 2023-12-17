open Core

let parse input =
  String.split_lines input

let solve lines =
  Printf.printf "%d\n" (List.length lines)

let main input = In_channel.read_all input |> parse |> solve
