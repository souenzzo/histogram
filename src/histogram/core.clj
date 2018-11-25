(ns histogram.core
  (:require [clojure.spec.alpha :as s])
  (:import (clojure.lang PersistentTreeMap)))

(defn group-intervals
  [values step]
  (if (empty? values)
    values
    (loop [[[k v] & rest-ks :as ks] (seq values)
           acc 0
           gid k
           limit (+ gid step)
           grouped (sorted-map)]
      (cond
        (empty? ks) (assoc grouped gid acc)
        (> k limit) (recur ks
                           0
                           (+ gid step)
                           (+ limit step)
                           (if (zero? acc)
                             grouped
                             (assoc grouped gid acc)))
        :else (recur rest-ks
                     (+ acc v)
                     gid
                     limit
                     grouped)))))

(s/fdef group-intervals
        :args (s/cat :values (s/map-of number? pos-int?
                                       :kind sorted?)
                     :step pos?)
        :ret (s/map-of number? pos-int?
                       :kind sorted?))

(defprotocol IHistogramValue
  (combine [this other])
  (normalize [this step]))

(extend-protocol IHistogramValue
  PersistentTreeMap
  (combine [this other]
    (merge-with + this other))
  (normalize [this step]
    (group-intervals this step)))

(defn histogram-value?
  [x]
  (satisfies? IHistogramValue x))

(defn histogram-value
  [n]
  (if (nil? n)
    (sorted-map)
    (sorted-map n 1)))

(s/fdef histogram-value
        :args (s/cat :n (s/nilable number?))
        :ret histogram-value?)

(defn -main
  [& args])
