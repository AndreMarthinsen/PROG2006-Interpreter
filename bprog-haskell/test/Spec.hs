-- For the Property Testing we need the following language extension
{-# LANGUAGE TemplateHaskell  #-}
-- And for our negative literals this comes really useful
{-# LANGUAGE NegativeLiterals #-}

import Test.Hspec
import Data.Either.Combinators (fromRight)




officialTests :: SpecWith ()
officialTests =
  describe "official tests" $ do
    describe "literals" $ do
        t "3"                           "3"
        t "121231324135634563456363567" "121231324135634563456363567"
        t "1.0"                         "1.0"
        t "0.0"                         "0.0"
        t "-1"                          "-1"
        t "-1.1"                        "-1.1"
        t "False"                       "False"
        t "True"                        "True"
        t "[ [ ] [ ] ]"                 "[[],[]]"
        t "[ False [ ] True [ 1 2 ] ]"  "[False,[],True,[1,2]]"
        t "\" [ so { not if ] and } \"" "\"[ so { not if ] and }\""

    describe "quotation literals" $ do
        t "{ 20 10 + }"             "{ 20 10 + }"
        t "{ { print } exec }"      "{ { print } exec }"
        t "[ { + } { 10 + } { 20 10 + } ]"   "[{ + },{ 10 + },{ 20 10 + }]"

    describe "simple arithmetic" $ do
        t "1 1 +"               "2"
        t "10 20 *"             "200"
        t "20 2 div"            "10"
        t "20 2 /"              "10.0"

    describe "arithmetic with type coercion" $ do
        t "1 1.0 +"             "2.0"
        t "10 20.0 *"           "200.0"
        t "20 2.0 div"          "10"
        t "20.0 2.0 div"        "10"
        t "True 0 + False 0 + =="   "False" -- optional check if True and False are coerced differently

    describe "bool operations" $ do
        t "False False &&"      "False"
        t "False True ||"       "True"
        t "False not"           "True"
        t "True not"            "False"

    describe "comparisons" $ do
        t "20 10 <"             "False"
        t "20 10 >"             "True"
        t "20 10.0 >"           "True"
        t "20.0 20.0 >"         "False"
        t "10 10 =="            "True"
        t "10 10.0 =="          "True"
        t "True True =="        "True"
        t "True 40 40 == =="    "True"
        t "\" abba \" \" abba \" ==" "True"
        t "[ ] [ ] =="          "True"
        t "[ 1 2 ] [ 1 2 ] =="  "True"
        t " [ [ ] ] [ [ ] ] ==" "True"

    describe "stack operations" $ do
        t "10 20 swap pop"          "20"
        t "10 dup dup + swap pop"   "20"
        t "10 20 swap dup + div"    "1"

    describe "length" $ do
        t "\" hello \" length"              "5"
        t "\" hello world \" length"        "11"
        t "[ 1 2 3 [ ] ] length"            "4"
        t "{ 10 20 + } length"              "3"

    describe "String parsing" $ do
        t "\" 12 \" parseInteger"           "12"
        t "\" 12.34 \" parseFloat"          "12.34"
        t "\" adam bob charlie \" words"    "[\"adam\",\"bob\",\"charlie\"]"

    describe "lists" $ do
        t "[ 1 2 3 ]"           "[1,2,3]"
        t "[ 1 \" bob \" ]"     "[1,\"bob\"]"
        t "[ 1 2 ] empty"       "False"
        t "[ ] empty"           "True"
        t "[ 1 2 3 ] head"      "1"
        t "[ 1 2 3 ] length"    "3"
        t "[ 1 2 3 ] tail"      "[2,3]"
        t "1 [ ] cons"          "[1]"
        t "1 [ 2 3 ] cons"      "[1,2,3]"
        t "[ 1 ] [ 2 3 ] append" "[1,2,3]"
        t "[ 1 2 ] [ ] append"  "[1,2]"
        t "[ 1 ] [ 2 3 ] cons"  "[[1],2,3]"

    describe "list quotations" $ do
        t "[ 1 2 3 ] map { 10 * }"                              "[10,20,30]"
        t "[ 1 2 3 ] map { 1 + }"                               "[2,3,4]"
        t "[ 1 2 3 4 ] map { dup 2 > if { 10 * } { 2 * } }"     "[2,4,30,40]"
        t "[ 1 2 3 4 ] each { 10 * } + + +"                     "100"
        t "[ 1 2 3 4 ] 0 foldl { + }"                           "10"
        t "[ 2 5 ] 20 foldl { div }"                            "2"

        {-- note no { } needed for 1 instruction code -}
        t "[ \" 1 \" \" 2 \" \" 3 \" ] each { parseInteger } [ ] cons cons cons" "[1,2,3]"
        t "[ \" 1 \" \" 2 \" \" 3 \" ] each parseInteger [ ] 3 times cons"       "[1,2,3]"
        t "[ 1 2 3 4 ] 0 foldl +"                               "10"
        t "[ 2 5 ] 20 foldl div"                                "2"

    describe "assignments" $ do
        t "age"                             "age"
        t "age 10 := age"                   "10"
        t "10 age swap := age"              "10"
        t "[ 1 2 3 ] list swap := list"     "[1,2,3]"
        t "age 20 := [ 10 age ]"            "[10,20]"

        t "inc { 1 + } fun 1 inc"           "2"
        t "mul10 { 10 * } fun inc { 1 + } fun 10 inc mul10" "110"

    describe "quotations" $ do
        t "{ 20 10 + } exec"                "30"
        t "10 { 20 + } exec"                "30"
        t "10 20 { + } exec"                "30"
        t "{ { 10 20 + } exec } exec"       "30"
        t "{ { 10 20 + } exec 20 + } exec"  "50"

    describe "if with quotation blocks" $ do
        t "True if { 20 } { }"               "20"
        t "True if { 20 10 + } { 3 }"        "30"
        t "10 5 5 == if { 10 + } { 100 + }"  "20"
        t "False if { } { 45 }"              "45"
        t "True if { False if { 50 } { 100 } } { 30 }" "100"

    describe "if without quotation, more ergonomic expressions" $ do
        t "True if 20 { }"                 "20"
        t "True if { 20 10 + } 3"          "30"
        t "10 10 5 5 == if + { 100 + }"    "20"
        t "False if { } 45"                "45"
        t "True if { False if 50 100 } 30" "100"

    describe "times" $ do
        t "1 times { 100 50 + }"                               "150"
        t "5 times { 1 } [ ] 5 times { cons } 0 foldl { + }"   "5"
        t "5 times 1     [ ] 5 times   cons   0 foldl   +  "   "5"
        t "5 times { 10 } + + + +"                             "50"
        t "5 times 10 4 times +"                               "50"

    describe "loop" $ do
        t "1 loop { dup 4 > } { dup 1 + } [ ] 5 times { cons }"         "[1,2,3,4,5]"
        t "1 loop { dup 4 > } { dup 1 + } [ ] 5 times   cons  "         "[1,2,3,4,5]"
        t "[ 1 ] loop { dup length 9 > }  { dup head 1 + swap cons }"   "[10,9,8,7,6,5,4,3,2,1]"

        t "odd { dup 2 div swap 2 / == if False True } fun \
        \  2 odd"                                                       "False"

        t "odd { dup 2 div swap 2 / == if False True } fun \
        \ 3 odd"                                                        "True"

        t "toList { [ ] swap times cons } fun \
        \ 1 2 3 4 \
        \4 toList"                                                      "[1,2,3,4]"

        t "gen1toNum { max swap := 1 loop { dup max > } { dup 1 + } } fun \
        \ 3 gen1toNum + + +"                                            "10"

        t "odd { dup 2 div swap 2 / == if False True } fun \
         \ toList { [ ] swap times cons } fun \
         \ gen1toNum { max swap := 1 loop { dup max > } { dup 1 + } } fun \
         \ 4 gen1toNum 5 toList map odd"                                "[True,False,True,False,True]"

    describe "extra programs" $ do
        t "drop { times tail } fun \
        \  [ 1 2 3 4 5 ] 3 drop"         "[4,5]"


-- | TODO rewrite this function called `t` such that it takes the input string, and expected output string,
-- and asks your interpreter, if executing the program from the Input String (inputString) produces the output
-- represented by the output string.
-- The current implementation is using Mariusz's API
t :: String -> String -> SpecWith ()
t inputString outputString = it i $ show (fromRight (VString "error") (parseAndExecute inputString)) `shouldBe` outputString
        
main :: IO ()
main = do
  -- Let us run first all the doctests from our source files
  doctest ["-isrc", "app/Main.hs"]

  hspec $ do
    officialTests
