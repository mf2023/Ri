// Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//
// This file is part of Ri.
// The Ri project belongs to the Dunimd Team.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// You may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package com.dunimd.ri.observability;

import java.util.Map;

/**
 * A single metric sample with timestamp, value, and labels.
 */
public class RiMetricSample {
    private final long timestamp;
    private final double value;
    private final Map<String, String> labels;

    public RiMetricSample(long timestamp, double value, Map<String, String> labels) {
        this.timestamp = timestamp;
        this.value = value;
        this.labels = labels;
    }

    public long getTimestamp() {
        return timestamp;
    }

    public double getValue() {
        return value;
    }

    public Map<String, String> getLabels() {
        return labels;
    }
}
