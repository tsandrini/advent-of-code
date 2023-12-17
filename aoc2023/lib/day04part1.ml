open Core
open Utils

let solve lines =
  List.map lines ~f:(fun line ->
      line |> String.split ~on:':' |> List.tl_exn |> List.hd_exn |> String.strip
      |> UString.reduce_multiple_whitespaces |> String.split ~on:'|'
      |> List.map ~f:String.strip
      |> List.map ~f:(String.split ~on:' ')
      |> List.map ~f:(List.map ~f:Int.of_string)
      |> UList.to_tuple_exn
      |> (fun (x, y) -> List.filter ~f:(fun el -> List.mem x el ~equal:( = )) y)
      |> List.length |> fun num -> (if num < 2 then num else Int.pow 2 (num - 1)))
  |> UList.fold_sum

let main input = In_channel.read_lines input |> solve |> Printf.printf "%d\n"

let%test "Day04 - test example input" =
  solve
    [
      "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
      "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
      "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
      "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
      "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36";
      "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    ]
  = 13
