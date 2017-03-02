$(function() {
    var ctx = document.getElementById("chart");
    var chartData = $(ctx).data('chart-data');
    var labels = $.map(chartData, function(x) { return x.date; });
    var failedIpCounts = $.map(chartData, function(x) {
        var n = 0;
        $.map(x.failed, function() { ++n });
        return n;
    });
    var failedCounts = $.map(chartData, function(x) {
        var n = 0;
        $.map(x.failed, function(v) { n += v });
        return n;
    });
    var oauthFailedCounts = $.map(chartData, function(x) { return x.oauth_failed });

    var myChart = new Chart(ctx, {
        //type: 'bar',
        type: 'line',
        data: {
            labels: labels,
            datasets: [
                { label: 'ログイン失敗IP数',
                  data: failedIpCounts,
                  borderWidth: 1
                },
                { label: 'ログイン失敗数',
                  data: failedCounts,
                  borderWidth: 1
                },
                { label: 'OAuth失敗数',
                  data: oauthFailedCounts,
                  borderWidth: 1
                }
            ]
        },
        options: {
            scales: {
                yAxes: [{
                    ticks: {
                        beginAtZero:true
                    }
                }]
            }
        }
    });
});
