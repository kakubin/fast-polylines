# frozen_string_literal: true

require_relative "fast_polylines/fast_polylines"

module FastPolylines::Encoder
  # @deprecated Use {FastPolylines.encode} instead.
  def self.encode(points, precision = 1e5)
    warn "Deprecated use of `FastPolylines::Encoder.encode`, " \
         "use `FastPolylines.encode`."
    FastPolylines.encode(points, Math.log10(precision))
  end
end

module FastPolylines::Decoder
  # @deprecated Use {FastPolylines.decode} instead.
  def self.decode(polyline, precision = 1e5)
    warn "Deprecated use of `FastPolylines::Decoder.decode`, " \
         "use `FastPolylines.decode`."
    FastPolylines.decode(polyline, Math.log10(precision))
  end
end
