open Core
open Utils

type card =
  | A
  | K
  | Q
  | J
  | T
  | Nine
  | Eight
  | Seven
  | Six
  | Five
  | Four
  | Three
  | Two
[@@deriving sexp, compare]

let card_of_char = function
  | 'A' -> A
  | 'K' -> K
  | 'Q' -> Q
  | 'J' -> J
  | 'T' -> T
  | '9' -> Nine
  | '8' -> Eight
  | '7' -> Seven
  | '6' -> Six
  | '5' -> Five
  | '4' -> Four
  | '3' -> Three
  | '2' -> Two
  | _ -> failwith "Invalid card"

type hand_type =
  | FiveOfAKind
  | FourOfAKind
  | FullHouse
  | ThreeOfAKind
  | TwoPair
  | OnePair
  | HighCard
[@@deriving sexp, compare]

type hand = { hand_type : hand_type; cards : card list }

let hand_of_card_list = function
  | [ _; _; _; _; _ ] as cards ->
      let unique = List.dedup_and_sort ~compare:compare_card cards in
      let unique_num = List.length unique in
      let freq_count =
        List.map unique ~f:(fun card ->
            List.count cards ~f:(fun card' -> compare_card card card' = 0))
        |> List.sort ~compare |> List.rev
      in
      if unique_num = 1 then { hand_type = FiveOfAKind; cards }
      else if unique_num = 2 then
        if List.hd_exn freq_count = 4 then { hand_type = FourOfAKind; cards }
        else { hand_type = FullHouse; cards }
      else if unique_num = 3 then
        if List.hd_exn freq_count = 3 then { hand_type = ThreeOfAKind; cards }
        else { hand_type = TwoPair; cards }
      else if unique_num = 4 then { hand_type = OnePair; cards }
      else { hand_type = HighCard; cards }
  | _ -> failwith "Invalid cards"

let compare_hands (first, _) (second, _) =
  let cmp_hand = compare_hand_type first.hand_type second.hand_type in
  if cmp_hand = 0 then
    let cmp_individual_cards =
      List.map2_exn first.cards second.cards ~f:compare_card
      |> List.filter ~f:(fun x -> x <> 0)
    in
    if List.length cmp_individual_cards <> 0 then List.hd_exn cmp_individual_cards
    else 0
  else cmp_hand

let solve lines =
  List.map lines ~f:(fun line ->
      String.strip line |> String.split ~on:' ' |> UList.to_tuple_exn
      |> fun (card, bid) ->
      ( String.to_list card |> List.map ~f:card_of_char |> hand_of_card_list,
        Int.of_string bid ))
  |> List.sort ~compare:compare_hands
  |> List.rev
  |> List.mapi ~f:(fun idx (_, bid) -> (idx + 1) * bid)
  |> UList.fold_sum

let main input =
  In_channel.read_lines input |> solve |> Printf.printf "Result: %d\n"

let%test "Day07part1 - test example input" =
  solve [ " 32T3K 765"; "T55J5 684"; "KK677 28"; "KTJJT 220"; "QQQJA 483" ]
  = 6440
