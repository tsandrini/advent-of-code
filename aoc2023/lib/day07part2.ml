(* Utop line *)
(* #use "topfind";; #require "core";; #require "ppx_jane";; #require "ppx_deriving";; #require "ppx_deriving.std";; #require "ppx_inline_test";; #require "ppx_hash";; #require "ppx_sexp_conv";; #require "ppx_compare";; *)

module List' = List
module String' = String
open Core

type card =
  | A
  | K
  | Q
  | T
  | Nine
  | Eight
  | Seven
  | Six
  | Five
  | Four
  | Three
  | Two
  | J
[@@deriving sexp, compare]

let all_cards =
  [ A; K; Q; T; Nine; Eight; Seven; Six; Five; Four; Three; Two; J ]

let card_of_char = function
  | 'A' -> A
  | 'K' -> K
  | 'Q' -> Q
  | 'T' -> T
  | '9' -> Nine
  | '8' -> Eight
  | '7' -> Seven
  | '6' -> Six
  | '5' -> Five
  | '4' -> Four
  | '3' -> Three
  | '2' -> Two
  | 'J' -> J
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

let compare_hands first second =
  let cmp_hand = compare_hand_type first.hand_type second.hand_type in
  if cmp_hand = 0 then
    let cmp_individual_cards =
      List.map2_exn first.cards second.cards ~f:compare_card
      |> List.filter ~f:(fun x -> x <> 0)
    in
    if List.length cmp_individual_cards <> 0 then List'.hd cmp_individual_cards
    else 0
  else cmp_hand

let rec hand_of_card_list ?(joker = false) = function
  | [ _; _; _; _; _ ] as cards ->
      let unique = List.dedup_and_sort ~compare:compare_card cards in
      let unique_num = List.length unique in
      let freq_count =
        List.map unique ~f:(fun card ->
            List.count cards ~f:(fun card' -> compare_card card card' = 0))
        |> List.sort ~compare |> List.rev
      in
      let jokers =
        Utils.elem_indices_of_list ~elem:J ~compare:compare_card cards
      in
      let joker_count = List.length jokers in
      if joker && joker_count <> 0 then
        let permutations =
          List.init joker_count ~f:(fun _ -> all_cards)
          |> Utils.rec_product_of_list
        in
        List.map permutations ~f:(fun perm ->
            let index_map = List.zip_exn jokers perm in
            let new_hand =
              List.mapi cards ~f:(fun idx elem ->
                  match List.Assoc.find index_map idx ~equal:Int.equal with
                  | Some new_elem -> new_elem
                  | None -> elem)
            in
            hand_of_card_list ~joker:false new_hand)
        |> List.sort ~compare:compare_hands
        |> List'.hd
        |> fun wildcard -> { hand_type = wildcard.hand_type; cards }
      else if unique_num = 1 then { hand_type = FiveOfAKind; cards }
      else if unique_num = 2 then
        if List'.hd freq_count = 4 then { hand_type = FourOfAKind; cards }
        else { hand_type = FullHouse; cards }
      else if unique_num = 3 then
        if List'.hd freq_count = 3 then { hand_type = ThreeOfAKind; cards }
        else { hand_type = TwoPair; cards }
      else if unique_num = 4 then { hand_type = OnePair; cards }
      else { hand_type = HighCard; cards }
  | _ -> failwith "Invalid cards"

let solve lines =
  List.map lines ~f:(fun line ->
      String.strip line |> String.split ~on:' ' |> Utils.list_to_tuple
      |> fun (card, bid) ->
      ( String.to_list card |> List.map ~f:card_of_char
        |> hand_of_card_list ~joker:true,
        Int.of_string bid ))
  |> List.sort ~compare:(fun (first, _) (second, _) ->
         compare_hands first second)
  |> List.rev
  |> List.mapi ~f:(fun idx (_, bid) -> (idx + 1) * bid)
  |> List.fold_left ~f:( + ) ~init:0

let main input =
  In_channel.read_lines input |> solve |> Printf.printf "Result: %d\n"

let%test "Day07part2 - test example input" =
  solve [ " 32T3K 765"; "T55J5 684"; "KK677 28"; "KTJJT 220"; "QQQJA 483" ]
  = 5905
