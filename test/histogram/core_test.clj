(ns histogram.core-test
  (:require [clojure.test :refer [deftest]]
            [midje.sweet :refer :all]
            [histogram.core :as hist]
            [clojure.spec.alpha :as s]
            [clojure.test.check.generators :as gen]))

(def gen-histogram-value (gen/fmap #(hist/histogram-value %)
                                   (gen/double* {:NaN?      false
                                                 :infinite? false})))

(deftest combine-test
  (let [hist-1 (gen/generate gen-histogram-value)
        hist-2 (gen/generate gen-histogram-value)
        hist-1+2 (hist/combine hist-1 hist-2)
        hist-2+1 (hist/combine hist-2 hist-1)
        hist-1+1+1+2 (reduce hist/combine [hist-1 hist-1 hist-1 hist-2])]
    (fact
      (= hist-1 hist-1) => true)
    (fact
      (= hist-1+2 hist-2+1) => true)
    (fact
      (sorted? hist-1+1+1+2)
      => true)
    (fact
      (counted? hist-1+1+1+2)
      => true)
    (fact
      "Should be something between [0,2] results"
      (<= 0 (->> hist-1+1+1+2 keys count) 2)
      => true)
    (fact
      "Final values should be the same the union of all initial values"
      (= (->> hist-1+1+1+2 keys set)
         (into #{} (comp (map keys) cat) [hist-1 hist-2]))
      => true)))


(deftest groupping-intervals
  (let [data {2  55
              3  11
              4  33
              6  65
              7  44
              12 1}
        step 3]
    (fact
      (hist/group-intervals data step)
      => {2  99
          5  109
          11 1})))
